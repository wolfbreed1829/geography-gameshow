import { test, expect } from '@playwright/test';
import { readFileSync } from 'node:fs';

const routes = ['index', 'programs', 'programs-scheduling', 'geography', 'art', 'geography-photos', 'art-photos', 'geography-videos', 'art-videos', 'in-action', 'meet-neal', 'challenge', 'planning', 'references', 'booking'];
const sizes = [[1440, 1000], [1280, 800], [390, 844], [320, 568], [844, 390]];
const photos = JSON.parse(readFileSync(new URL('../docs/PROGRAM-PHOTOS.json', import.meta.url), 'utf8'));
const videos = JSON.parse(readFileSync(new URL('../docs/PROGRAM-VIDEOS.json', import.meta.url), 'utf8'));

async function settle(page) {
  await page.evaluate(() => document.fonts.ready);
  await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
}
async function fits(page) {
  await settle(page);
  const bounds = await page.evaluate(() => ({
    width: innerWidth, height: innerHeight,
    scrollWidth: document.documentElement.scrollWidth, scrollHeight: document.documentElement.scrollHeight,
    readers: [...document.querySelectorAll('.reader-body')].filter(e => e.clientHeight > 0).map(e => ({ available: e.clientHeight, text: e.querySelector('.reader-text').getBoundingClientRect().height })),
  }));
  expect(bounds.scrollWidth, `${page.url()}: horizontal overflow`).toBeLessThanOrEqual(bounds.width);
  expect(bounds.scrollHeight, `${page.url()}: vertical overflow`).toBeLessThanOrEqual(bounds.height);
  for (const reader of bounds.readers) expect(reader.text, 'Reading text must fit without clipping').toBeLessThanOrEqual(reader.available + 1);
}
async function section(page, label) {
  await page.locator('.screen-deck').waitFor();
  const picker = page.getByRole('combobox', { name: 'Page section', exact: true });
  if (await picker.isVisible()) await picker.selectOption({ label });
  else await page.getByRole('group', { name: 'Page sections', exact: true }).getByRole('button', { name: label, exact: true }).click();
  await settle(page);
}
async function readAll(page) {
  const reader = page.locator('.screen-reader:visible');
  await expect(reader.locator('.reader-text:not(.reader-probe)')).not.toBeEmpty();
  const next = reader.getByRole('button', { name: 'Next', exact: true });
  let text = '';
  for (let i = 0; i < 200; i++) {
    await fits(page);
    text += await reader.locator('.reader-text:not(.reader-probe)').textContent();
    if (await next.isDisabled()) return text;
    await next.click();
  }
  throw new Error('Reading pagination did not terminate');
}

for (const route of routes) {
  test(`${route}: one screen, usable sections, and visible booking`, async ({ page }) => {
    test.setTimeout(90000);
    const errors = [];
    page.on('pageerror', e => errors.push(e.message));
    const response = await page.goto(`${route}.html`);
    expect(response.status()).toBe(200);
    await expect(page.locator('main h1')).toHaveCount(1);
    await expect(page.locator('main h1')).toBeVisible();
    await expect(page.locator('body')).not.toContainText(/Anatomy/i);
    await expect(page.locator('a[href*="anatomy"]')).toHaveCount(0);
    for (const [width, height] of sizes) {
      await page.setViewportSize({ width, height });
      await fits(page);
      const booking = width <= 900 ? page.locator('.mobile-book') : page.locator('#main-nav').getByRole('link', { name: 'Request a Show', exact: true });
      await expect(booking).toBeInViewport();
      if (route !== 'index') {
        const labels = await page.locator('.screen-deck > .screen-tabs > button').allTextContents();
        for (const label of labels) {
          await section(page, label);
          await fits(page);
          const panel = page.locator('.screen-panel:visible');
          await expect(panel).toHaveCount(1);
        }
        if (labels.length) await section(page, labels[0]);
      }
    }
    expect(errors).toEqual([]);
  });
}

test('home retains its large logo and has no extra content below it', async ({ page }) => {
  await page.goto('index.html');
  await expect(page.locator('.landing-logo')).toBeVisible();
  await expect(page.locator('.landing-hero h1')).toHaveText('World maps illustrated from memory');
  await expect(page.locator('.site-copyright')).toHaveText('(c) 1992 Neal Nichols Jr. geographygameshow.com');
  await expect(page.locator('.landing-hero > p')).toHaveText('Educational performances for all ages');
  await expect(page.locator('.home-evidence')).toHaveCount(0);
  await expect(page.locator('.landing-actions').getByRole('link', { name: 'Request a Show' })).toBeInViewport();
  await page.locator('.landing-actions').getByRole('link', { name: 'View Programs' }).click();
  await expect(page).toHaveURL(/programs.html$/);
  await expect(page.locator('.program-choice:visible img')).toHaveAttribute('src', 'public/programs/geography-sketchbook.webp');
  await expect(page.locator('.program-choice:visible img')).toHaveCSS('object-fit', 'contain');
  await page.locator('.program-choice:visible').click();
  await expect(page).toHaveURL(/geography.html$/);
  await section(page, 'Artwork');
  await expect(page.locator('.art-display:visible img')).toHaveAttribute('src', 'public/photos/complete-world-map.webp');
  await expect(page.locator('.art-display.original-drawing:visible')).toBeVisible();
});

test('color washes react to the pointer, press, pause, and reduced motion', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'no-preference' });
  await page.goto('index.html');
  const request = page.locator('.landing-actions .button');
  await expect(request).toHaveText('Request a Show');
  await expect(request).toHaveCSS('animation-name', 'color-cascade');
  await expect(page.locator('.landing-hero h1')).toHaveCSS('animation-name', 'lettering-reflection');
  const position = await request.evaluate(e => getComputedStyle(e).backgroundPosition);
  await expect.poll(() => request.evaluate(e => getComputedStyle(e).backgroundPosition)).not.toBe(position);
  const washTransform = await request.evaluate(e => getComputedStyle(e, '::before').transform);
  await expect.poll(() => request.evaluate(e => getComputedStyle(e, '::before').transform)).not.toBe(washTransform);
  await request.hover({ position: { x: 20, y: 15 } });
  expect(await request.evaluate(e => e.style.getPropertyValue('--pointer-x'))).not.toBe('');
  await expect.poll(() => request.evaluate(e => Number(getComputedStyle(e, '::after').opacity))).toBe(1);
  const raised = await request.evaluate(e => getComputedStyle(e).transform);
  await page.mouse.down();
  await expect.poll(() => request.evaluate(e => getComputedStyle(e).transform)).not.toBe(raised);
  await page.mouse.move(10, 10);
  await page.mouse.up();
  await page.getByRole('button', { name: 'Pause animations', exact: true }).click();
  await expect(request).toHaveCSS('animation-play-state', 'paused');
  expect(await request.evaluate(e => getComputedStyle(e, '::before').animationPlayState)).toBe('paused');
  await expect(page.locator('.landing-hero h1')).toHaveCSS('animation-play-state', 'paused');
  await page.getByRole('button', { name: 'Resume animations', exact: true }).click();
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await expect(request).toHaveCSS('animation-name', 'none');
  expect(await request.evaluate(e => getComputedStyle(e, '::before').animationName)).toBe('none');
  await expect(page.locator('.landing-hero h1')).toHaveCSS('animation-name', 'none');
  await request.focus();
  await expect(request).toBeFocused();
});

const guidePage = 'programs-scheduling.html';
const guideName = 'Programs & Scheduling';
const programsByGrade = [
  ['Kindergarten', ['Drawing and Art', 'Local Regional Show', 'Cape Cod Introduction'], '45–60 minutes'],
  ['Grade 1', ['New England and the Pilgrims', 'USA Map Introduction'], '60 minutes'],
  ['Grade 2', ['World Map Introduction'], '75 minutes'],
  ['Grade 3', ['Massachusetts History', 'USA Capitals / National Parks'], '75 minutes'],
  ['Grade 4', ['USA History'], '90 minutes'],
  ['Grade 5', ['The Original Colonies'], '90 minutes'],
  ['Grade 6', ['Ancient / Middle East'], '90 minutes'],
  ['Grade 7', ['World Map Challenge'], '90 minutes'],
];

test('the main navigation opens the scheduling guide and exposes the published booking email', async ({ page, baseURL }) => {
  await page.goto('./');
  const navigation = page.getByRole('navigation', { name: 'Main navigation' });
  const programs = navigation.locator('.program-menu summary');
  await programs.focus();
  await page.keyboard.press('Enter');
  const guideLink = navigation.getByRole('link', { name: guideName, exact: true });
  await guideLink.focus();
  await page.keyboard.press('Enter');
  await expect(page).toHaveURL(new URL(guidePage, baseURL).href);
  await expect(page).toHaveTitle('Programs & Scheduling | The Geography Gameshow');
  await expect(navigation.locator('.program-menu a[aria-current="page"]')).toHaveAttribute('href', guidePage);
  await expect(page.getByRole('heading', { level: 1, name: guideName })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'A World of Learning, Illustrated from Memory' })).toBeVisible();
  await page.locator('main img:visible').evaluateAll(images => Promise.all(images.map(image => image.decode())));
  await expect(page.getByRole('link', { name: 'Ask About a Visit', exact: true }))
    .toHaveAttribute('href', 'mailto:geographygameshow@yahoo.com');
});

test('each grade has its specified program choices and running time', async ({ page }) => {
  await page.goto(guidePage);
  await section(page, 'Programs by Grade');
  const table = page.getByRole('table');
  await expect(table.getByRole('columnheader')).toHaveText(['Suggested Grade', 'Available Programs', 'Running Time']);
  await expect(table.getByRole('rowheader')).toHaveText(programsByGrade.map(([grade]) => grade));
  for (const [grade, choices, duration] of programsByGrade) {
    const row = table.getByRole('row').filter({ has: page.getByRole('rowheader', { name: grade, exact: true }) });
    const cells = row.getByRole('cell');
    await expect(cells.nth(0).getByRole('listitem')).toHaveText(choices);
    await expect(cells.nth(1)).toHaveText(duration);
  }
  await page.setViewportSize({ width: 320, height: 568 });
  const finalChoice = table.getByText('World Map Challenge', { exact: true });
  await finalChoice.scrollIntoViewIfNeeded();
  await expect(finalChoice).toBeInViewport();
  await section(page, 'Additional Programs');
  const additional = page.getByRole('region', { name: 'More Places. More Stories. More Ways to Learn.' });
  await expect(additional.getByRole('listitem')).toHaveText([
    'Alaska History', 'Texas History', '31 States of Mexico', 'Latin America', 'Europe Shows', 'Britain 1066', 'World Tectonic Plates and Disaster',
  ]);
  await additional.locator('img').evaluateAll(images => Promise.all(images.map(image => image.decode())));
});

test('sample visits preserve the order, breaks and durations, with responsive day columns', async ({ page }) => {
  await page.goto(guidePage);
  await section(page, 'Visit Schedules');
  const planning = page.getByRole('region', { name: 'Plan a Visit Around Your School Day' });
  await expect(planning).toContainText('Allow at least 15 minutes between shows');
  await expect(planning).toContainText('Neal arrives 45–60 minutes before the first show.');
  await expect(planning).toContainText('Final arrangements are coordinated for each visit.');
  const oneDay = page.getByRole('region', { name: 'Example One-Day Visit', exact: true });
  const twoDay = page.getByRole('region', { name: 'Example Two-Day Visit', exact: true });
  const day1 = twoDay.getByRole('region', { name: 'Day 1', exact: true });
  const day2 = twoDay.getByRole('region', { name: 'Day 2', exact: true });
  const changeover = ['Changeover', 'at least 15 minutes'];
  const lunch = ['Lunch'];
  const worldMap = ['Grade 2', 'World Map Introduction', '75 minutes'];
  const massachusetts = ['Grade 3', 'Massachusetts History', '75 minutes'];
  const usaHistory = ['Grade 4', 'USA History', '90 minutes'];
  const schedules = [
    [oneDay, [worldMap, changeover, massachusetts, lunch, usaHistory]],
    [day1, [['Kindergarten', 'Drawing and Art', '45–60 minutes'], changeover, worldMap, lunch, usaHistory]],
    [day2, [['Grade 1', 'New England and the Pilgrims', '60 minutes'], changeover, massachusetts, lunch, ['Grade 5', 'The Original Colonies', '90 minutes']]],
  ];
  for (const [day, entries] of schedules) {
    const rows = day.locator('ol > li');
    const expected = entries.map(parts => parts.join(' '));
    await expect(rows).toHaveText(expected);
  }
  const firstColumn = await day1.boundingBox();
  const secondColumn = await day2.boundingBox();
  expect(secondColumn.x).toBeGreaterThanOrEqual(firstColumn.x + firstColumn.width - 1);
  await page.setViewportSize({ width: 390, height: 844 });
  const firstCard = await day1.boundingBox();
  const secondCard = await day2.boundingBox();
  expect(secondCard.y).toBeGreaterThanOrEqual(firstCard.y + firstCard.height - 1);
  const lastShow = day2.getByText('The Original Colonies', { exact: true });
  await lastShow.scrollIntoViewIfNeeded();
  await expect(lastShow).toBeInViewport();
});

test('supplied curriculum retains all eight grades, titles, and running times', async ({ page }) => {
  await page.goto('geography.html');
  await section(page, 'Curriculum');
  await page.setViewportSize({ width: 320, height: 568 });
  for (const [i, [grade, titles, time]] of programsByGrade.entries()) {
    await page.getByRole('combobox', { name: 'Choose a grade' }).selectOption(String(i));
    await expect(page.locator('.curriculum-card')).toContainText(i === 0 ? 'Grade K' : grade);
    for (const title of titles) await expect(page.locator('.curriculum-card')).toContainText(title);
    await expect(page.locator('.curriculum-card')).toContainText(time);
    await fits(page);
  }
  await section(page, 'Program outline');
  const outline = await readAll(page);
  for (const phrase of ['fundraising shows', 'Montessori schools', 'multi-age audiences', 'Alaska History', '31 States of Mexico', 'Britain 1066', 'World Tectonic Plates and Disaster']) expect(outline).toContain(phrase);
});

test('scheduling preserves arrival, intervals, and both supplied schedule examples', async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 568 });
  await page.goto('planning.html');
  await section(page, 'Scheduling & show length');
  const scheduling = await readAll(page);
  expect(scheduling).toContain('15 minutes');
  expect(scheduling).toContain('45–60 minutes before');
  expect(scheduling).not.toContain('30–45');
  await section(page, 'One-day schedule');
  const one = await readAll(page);
  expect(one).toContain('Grade 2 · World Map Introduction · 75 minutes');
  expect(one).toContain('Grade 4 · USA History · 90 minutes');
  await section(page, 'Two-day schedule');
  const two = await readAll(page);
  expect(two).toContain('Day 1');
  expect(two).toContain('Day 2');
  expect(two).toContain('Grade 5 · The Original Colonies · 90 minutes');
});

for (const slug of ['geography', 'art']) {
  test(`${slug}: every original photo remains selectable and visible`, async ({ page }) => {
    test.setTimeout(90000);
    await page.setViewportSize({ width: 320, height: 568 });
    await page.goto(`${slug}-photos.html`);
    const collection = photos.filter(p => p.program === slug);
    const picker = page.locator('#gallery-photo-select');
    await expect(picker.locator('option')).toHaveCount(collection.length);
    for (let i = 0; i < collection.length; i++) {
      await picker.selectOption(String(i));
      await expect(page.locator('.gallery-image')).toHaveAttribute('src', `public/photos/${collection[i].file}.webp`);
      await page.locator('.gallery-image').evaluate(image => image.decode());
      await fits(page);
      const box = await page.locator('.gallery-image').boundingBox();
      expect(box.height).toBeGreaterThan(30);
      expect(box.y + box.height).toBeLessThan(568);
    }
    await page.getByRole('button', { name: 'Next photo', exact: true }).click();
    await expect(picker).toHaveValue('0');
  });
  test(`${slug}: verified videos still play on demand with one player`, async ({ page }) => {
    await page.route('https://www.youtube-nocookie.com/embed/**', route => route.fulfill({ contentType: 'text/html', body: '<html><body>Player</body></html>' }));
    await page.goto(`${slug}-videos.html`);
    await expect(page.locator('iframe')).toHaveCount(0);
    const collection = videos.filter(v => v.program === slug);
    const picker = page.getByRole('combobox', { name: 'Choose a video', exact: true });
    await expect(picker.locator('option')).toHaveCount(collection.length);
    for (let i = 0; i < collection.length; i++) {
      await picker.selectOption(String(i));
      await page.locator('.video-start').click();
      await expect(page.locator('iframe')).toHaveCount(1);
      await expect(page.locator('iframe')).toHaveAttribute('src', new RegExp(collection[i].id));
    }
    await section(page, 'Find a video');
    await page.getByRole('searchbox').fill('no-such-video');
    await expect(page.getByRole('status')).toHaveText('0 videos');
    await page.getByRole('button', { name: 'Show all videos' }).click();
    await expect(page.getByRole('combobox', { name: 'Search results' }).locator('option')).toHaveCount(collection.length + 1);
  });
}

test('references retain the video, testimonial, publications, and all 38 sources', async ({ page }) => {
  await page.route('https://www.youtube-nocookie.com/embed/**', route => route.fulfill({ contentType: 'text/html', body: '<html><body>Player</body></html>' }));
  await page.goto('references.html');
  await page.getByRole('button', { name: 'Play the Geography Gameshow introduction' }).click();
  await expect(page.locator('.featured-player iframe')).toHaveCount(1);
  await section(page, 'School story');
  expect(await readAll(page)).toContain('dynamic and fascinating');
  await section(page, 'Featured in');
  const press = await readAll(page);
  for (const name of ['The Provincetown Independent', 'The Nome Nugget', 'Overdrive']) expect(press).toContain(name);
  await section(page, 'Directory');
  const picker = page.getByRole('combobox', { name: 'Choose a reference', exact: true });
  await expect(picker.locator('option')).toHaveCount(38);
  const names = await picker.locator('option').allTextContents();
  expect(new Set(names).size).toBe(38);
  await page.setViewportSize({ width: 320, height: 568 });
  for (let i = 0; i < names.length; i++) {
    await picker.selectOption(String(i));
    await expect(page.locator('.reference-card h2')).toHaveText(names[i]);
    await fits(page);
  }
  await page.getByRole('searchbox').fill('Provincetown');
  await page.getByRole('button', { name: 'Read coverage', exact: true }).click();
  const text = await readAll(page);
  expect(text).toContain('Drawing Is Like Breathing');
  expect(text).toContain('A Life That’s Everywhere');
  expect(text).toContain('Publication titles and credits identify the sources.');
  await page.getByRole('button', { name: 'Back to references' }).click();
  await page.getByRole('searchbox').fill('nothing matches this');
  await expect(page.getByRole('heading', { name: 'No references found.' })).toBeVisible();
  await page.getByRole('button', { name: 'Show all references' }).click();
  await expect(picker.locator('option')).toHaveCount(38);
});

test('inquiry validates each step, retains entries, and paginates long text safely', async ({ page }) => {
  test.setTimeout(90000);
  await page.setViewportSize({ width: 320, height: 568 });
  await page.goto('booking.html?program=art');
  await page.getByRole('button', { name: 'Continue' }).click();
  await expect(page.locator('.step-label')).toContainText('Step 1');
  await page.getByLabel('School name', { exact: true }).fill('東京国際学校 & Arts');
  await page.getByLabel('Administrator or teacher name').fill('Yuki & Sam');
  await page.getByRole('button', { name: 'Continue' }).click();
  await page.getByLabel('Email address').fill('school@example.jp');
  await page.getByLabel('Phone number').fill('+81 (0)3 1234-5678');
  await fits(page);
  await page.getByRole('button', { name: 'Continue' }).click();
  await page.getByLabel('School address').fill('東京都新宿区 1-2-3\nTokyo 160-0022');
  await page.getByLabel('Country or territory').fill('日本 / Japan');
  await fits(page);
  await page.getByRole('button', { name: 'Continue' }).click();
  await expect(page.getByLabel('Program of interest')).toHaveValue('Art — concept inquiry');
  await expect(page.locator('option')).not.toContainText(['Anatomy']);
  const note = '<script>alert(1)</script> ' + 'X'.repeat(700);
  await page.getByLabel('Additional details').fill(note);
  const sent = [];
  page.on('request', r => { if (r.method() !== 'GET') sent.push(r.url()); });
  await page.getByRole('button', { name: 'Review inquiry' }).click();
  const review = await readAll(page);
  for (const value of ['東京国際学校 & Arts', 'Yuki & Sam', '+81 (0)3 1234-5678', '東京都新宿区', '日本 / Japan', note]) expect(review).toContain(value);
  expect(sent).toEqual([]);
  await expect(page.locator('.draft-preview script')).toHaveCount(0);
  await page.getByRole('button', { name: 'Edit inquiry' }).click();
  await expect(page.getByLabel('School name', { exact: true })).toHaveValue('東京国際学校 & Arts');
  await expect(page.locator('.draft-preview')).toHaveCount(0);
});

for (const [topic, answers] of [['Geography', [1, 0, 2]], ['Art', [1, 0, 2]]]) {
  test(`${topic}: challenge completes, scores, and resets without scrolling`, async ({ page }) => {
    await page.setViewportSize({ width: 320, height: 568 });
    await page.goto('challenge.html');
    await page.getByRole('group', { name: 'Challenge subject' }).getByRole('button', { name: topic, exact: true }).click();
    for (let i = 0; i < 3; i++) {
      await fits(page);
      await page.locator('.quiz-answers button').nth(answers[i]).click();
      await expect(page.locator('.quiz-feedback')).toContainText('You’ve got it!');
      await fits(page);
      await page.getByRole('button', { name: i === 2 ? 'See my result' : 'Next question' }).click();
    }
    await expect(page.locator('.quiz-result')).toContainText('3 out of 3');
    await fits(page);
    await page.getByRole('button', { name: 'Play again' }).click();
    await expect(page.locator('.quiz-progress')).toHaveText('Question 1 / 3');
  });
}

test('mobile navigation and motion controls remain usable', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('index.html');
  await page.getByRole('button', { name: 'Menu +' }).click();
  await page.locator('#main-nav').getByRole('link', { name: 'References', exact: true }).click();
  await expect(page.locator('#main-nav')).toBeHidden();
  await page.emulateMedia({ reducedMotion: 'no-preference' });
  await page.getByRole('button', { name: 'Pause animations' }).click();
  await expect(page.locator('.ocean-texture')).toHaveCSS('animation-play-state', 'paused');
  await page.goto('index.html');
  await expect(page.locator('.coastal-pan')).toHaveCSS('animation-play-state', 'paused');
  await page.getByRole('button', { name: 'Resume animations' }).click();
  await expect(page.locator('.coastal-pan')).toHaveCSS('animation-play-state', 'running');
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await expect(page.locator('.coastal-pan')).toHaveCSS('animation-name', 'none');
});

test('Programs dropdown links to Geography and Art on desktop and mobile', async ({ page }) => {
  await page.goto('index.html');
  const programs = page.locator('.program-menu');
  await programs.locator('summary').focus();
  await page.keyboard.press('Enter');
  await expect(programs.getByRole('link', { name: 'Geography', exact: true })).toBeVisible();
  await expect(programs.getByRole('link', { name: 'Art', exact: true })).toBeVisible();
  await programs.getByRole('link', { name: 'Geography', exact: true }).click();
  await expect(page.locator('main h1')).toHaveText('Geography');
  await page.setViewportSize({ width: 320, height: 568 });
  await page.getByRole('button', { name: 'Menu +' }).click();
  await programs.locator('summary').click();
  await programs.getByRole('link', { name: 'Art', exact: true }).click();
  await expect(page.locator('main h1')).toHaveText('Art');
  await expect(page.locator('.site-copyright')).toBeInViewport();
  const copyright = await page.locator('.site-copyright').boundingBox();
  expect(copyright.x).toBe(16);
  await fits(page);
});
