import { test, expect } from '@playwright/test';
import { readFileSync } from 'node:fs';

const approvedPhotos = JSON.parse(readFileSync(new URL('../docs/PROGRAM-PHOTOS.json', import.meta.url), 'utf8'));

test.beforeEach(async ({ page }) => {
  const startupErrors = [];
  page.on('pageerror', error => startupErrors.push(error.message));
  page.on('requestfailed', request => startupErrors.push(`${request.url()}: ${request.failure()?.errorText}`));
  await page.goto('./');
  try {
    await expect(page.getByRole('heading', { name: 'Big ideas. Drawn live.' })).toBeVisible();
  } catch (error) {
    console.error('Startup diagnostics:', { url: page.url(), startupErrors, body: (await page.locator('body').innerText()).slice(0, 600) });
    throw error;
  }
});

test('all local images and fonts load without browser errors', async ({ page }) => {
  test.setTimeout(90000); // Includes uncached CDN image/font transfers on a public deployment.
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  await page.reload();
  await page.getByRole('heading', { name: 'Big ideas. Drawn live.' }).waitFor();
  const assets = await page.evaluate(async () => {
    document.querySelectorAll('img').forEach(img => img.loading = 'eager');
    await Promise.all([...document.images].map(img => img.decode()));
    await document.fonts.ready;
    const ocean = new Image();
    ocean.src = new URL('public/ocean-background.webp', document.baseURI).href;
    await ocean.decode();
    return { images: [...document.images].every(img => img.naturalWidth > 0), fonts: [...document.fonts].every(font => font.status === 'loaded'), ocean: ocean.naturalWidth > 0 };
  });
  expect(assets).toEqual({ images: true, fonts: true, ocean: true });
  expect(errors).toEqual([]);
});

test('short homepage leads to separate program pages with working refresh and back', async ({ page }) => {
  await expect(page.locator('link#coastal-preload')).toHaveAttribute('as', 'image');
  await expect(page.locator('.show-card, .quiz-box, .inquiry-form')).toHaveCount(0);
  expect(await page.evaluate(() => document.documentElement.scrollHeight)).toBeLessThanOrEqual(1050);
  await page.locator('#main-nav').getByRole('link', { name: 'Programs', exact: true }).click();
  await expect(page).toHaveURL(/programs.html$/);
  await expect(page.locator('link#coastal-preload')).toHaveCount(0);
  await expect(page.locator('.show-card')).toHaveCount(3);
  await expect.poll(() => page.locator('.discovery-illustration').evaluateAll(images => images.every(image => image.complete && image.naturalWidth > 0))).toBe(true);
  await expect(page.locator('.discovery-status')).toHaveText(['LIVE EXPERIENCE', 'IN DEVELOPMENT', 'IN DEVELOPMENT']);
  await page.locator('.anatomy-card').click();
  await expect(page).toHaveURL(/anatomy.html$/);
  await expect(page.locator('h1')).toHaveText('Anatomy');
  await page.reload();
  await expect(page.locator('h1')).toHaveText('Anatomy');
  await page.goBack();
  await expect(page.locator('.show-card')).toHaveCount(3);
});

test('every page loads directly with its own title, valid links, and no published prices', async ({ page, request }) => {
  test.setTimeout(90000);
  const internalLinks = new Set();
  for (const slug of ['index', 'programs', 'geography', 'anatomy', 'art', 'geography-photos', 'anatomy-photos', 'art-photos', 'in-action', 'meet-neal', 'challenge', 'planning', 'booking', 'references']) {
    const response = await page.goto(`${slug}.html`);
    expect(response.status()).toBe(200);
    await expect(page.locator('main h1')).toHaveCount(1);
    await expect(page.locator('main h1')).toBeVisible();
    if (slug !== 'index') expect(await page.title()).toContain('| The Geography Gameshow');
    const content = await page.locator('body').innerText();
    expect(content).not.toMatch(/biology|pricing|\$\s*\d|deposit|cancellation fee/i);
    expect(content).not.toContain('\uFFFD');
    const links = await page.locator('a[href]').evaluateAll(els => els.map(el => el.href).filter(href => href.startsWith(location.origin)));
    links.forEach(href => internalLinks.add(href.split('#')[0]));
  }
  for (const href of internalLinks) expect((await request.get(href)).status(), href).toBe(200);
});

test('program subnavigation opens separate photo galleries with working controls and full images', async ({ page }) => {
  test.setTimeout(process.env.PREVIEW_URL ? 300000 : 180000);
  for (const [slug, name] of [['geography', 'Geography'], ['anatomy', 'Anatomy'], ['art', 'Art']]) {
    const photos = approvedPhotos.filter(photo => photo.program === slug);
    const count = photos.length;
    await page.goto(`${slug}.html`);
    const subnav = page.getByRole('navigation', { name: `${name} sections` });
    await expect(subnav.getByRole('link', { name: 'Overview' })).toHaveAttribute('aria-current', 'page');
    await expect(page.locator('.program-gallery')).toHaveCount(0);
    await subnav.getByRole('link', { name: /Photos/ }).click();
    await expect(page).toHaveURL(new RegExp(`${slug}-photos.html$`));
    await expect(subnav.getByRole('link', { name: /Photos/ })).toHaveAttribute('aria-current', 'page');
    const gallery = page.getByRole('region', { name: `${name} photo gallery` });
    const image = gallery.locator('.gallery-image');
    await expect(image).toHaveCSS('object-fit', 'contain');
    const sources = new Set();
    for (let i = 0; i < count; i++) {
      await image.evaluate(img => img.decode());
      sources.add(await image.getAttribute('src'));
      await expect(image).toHaveAttribute('src', `public/photos/${photos[i].file}.webp`);
      expect(await image.evaluate(img => [img.naturalWidth, img.naturalHeight])).toEqual(photos[i].size);
      const display = await image.boundingBox();
      expect(display.width).toBeLessThanOrEqual(photos[i].size[0]);
      expect(display.height).toBeLessThanOrEqual(photos[i].size[1]);
      if (count > 1) {
        await expect(gallery.locator('.gallery-count')).toHaveAttribute('aria-label', `Photo ${i + 1} of ${count}`);
        await expect(gallery.locator('.gallery-thumbnails button').nth(i)).toHaveAttribute('aria-pressed', 'true');
        await gallery.getByRole('button', { name: 'Next photo', exact: true }).click();
      }
    }
    expect(sources.size).toBe(count);
    if (count > 1) {
      await expect(gallery.locator('.gallery-count')).toHaveAttribute('aria-label', `Photo 1 of ${count}`);
      await gallery.getByRole('button', { name: 'Previous photo', exact: true }).click();
      await expect(gallery.locator('.gallery-count')).toHaveAttribute('aria-label', `Photo ${count} of ${count}`);
      const first = gallery.getByRole('button', { name: /^Show photo 1:/ });
      await first.focus();
      await page.keyboard.press('Enter');
      await expect(first).toHaveAttribute('aria-pressed', 'true');
      await expect(gallery.locator('.gallery-count')).toHaveAttribute('aria-label', `Photo 1 of ${count}`);
    } else {
      await expect(gallery.getByRole('button')).toHaveCount(0);
    }
    if (count > 8) {
      const jump = gallery.getByRole('combobox', { name: `Browse all ${count} photos` });
      await expect(jump.locator('option')).toHaveCount(count);
      await jump.selectOption(String(count - 1));
      await expect(gallery.locator('.gallery-count')).toHaveAttribute('aria-label', `Photo ${count} of ${count}`);
      await expect(image).toHaveAttribute('src', `public/photos/${photos[count - 1].file}.webp`);
      await gallery.getByRole('button', { name: 'Next photo', exact: true }).click();
      await expect(jump).toHaveValue('0');
      await gallery.getByRole('button', { name: /^Show photo 2:/ }).click();
      await expect(jump).toHaveValue('1');
    }
    await page.reload();
    await expect(gallery).toBeVisible();
    await subnav.getByRole('link', { name: 'Overview' }).click();
    await expect(page.locator('main h1')).toHaveText(name);
    await expect(page.locator('.program-gallery')).toHaveCount(0);
  }
  await page.goto('in-action.html');
  await expect(page.locator('.action-photo img')).toHaveAttribute('src', 'public/photos/world-in-progress.webp');
});

for (const [topic, answers] of [['Geography', [1, 0, 2]], ['Anatomy', [0, 1, 2]], ['Art', [1, 0, 2]]]) {
  test(`${topic} challenge scores correctly, prevents duplicate answers, and resets`, async ({ page }) => {
    await page.goto('challenge.html');
    const quiz = page.locator('.quiz-box');
    await quiz.getByRole('button', { name: topic, exact: true }).click();
    for (let i = 0; i < 3; i++) {
      await quiz.locator('.quiz-answers button').nth(answers[i]).click();
      await expect(quiz.locator('.quiz-feedback')).toContainText('You’ve got it!');
      await expect(quiz.locator('.quiz-answers button:disabled')).toHaveCount(3);
      await quiz.getByRole('button', { name: i === 2 ? 'See my result' : 'Next question' }).click();
    }
    await expect(quiz.locator('.quiz-result')).toContainText('3 out of 3');
    await quiz.getByRole('button', { name: 'Play again' }).click();
    await expect(quiz.locator('.quiz-progress')).toContainText('QUESTION 01 / 03');
    await expect(quiz.locator('.quiz-answers button:disabled')).toHaveCount(0);
    await quiz.locator('.quiz-answers button').nth((answers[0] + 1) % 3).click();
    await expect(quiz.locator('.quiz-feedback')).toContainText('A new thing to know');
    await expect(quiz.locator('.quiz-answers .correct')).toHaveCount(1);
    await quiz.getByRole('button', { name: topic === 'Art' ? 'Anatomy' : 'Art', exact: true }).click();
    await expect(quiz.locator('.quiz-progress')).toContainText('QUESTION 01 / 03');
    await expect(quiz.locator('.quiz-answers button:disabled')).toHaveCount(0);
  });
}

test('international inquiry validates, reviews locally, and clears stale previews', async ({ page }) => {
  await page.goto('booking.html?program=anatomy');
  await expect(page.getByLabel('Program of interest')).toHaveValue('Anatomy — concept inquiry');
  const review = page.getByRole('button', { name: 'Review inquiry' });
  await review.click();
  await expect(page.locator('.draft-preview')).toHaveCount(0);
  await page.getByLabel('School name', { exact: true }).fill('東京国際学校 & Arts');
  await page.getByLabel('Administrator or teacher name', { exact: true }).fill('佐藤 Yuki & Sam');
  await page.getByLabel('Email address', { exact: true }).fill('yuki+school@example.jp');
  await page.getByLabel('Phone number').fill('+81 (0)3 1234-5678 ext. 12');
  await page.getByLabel('School address', { exact: false }).fill('東京都新宿区 1-2-3\nTokyo 160-0022');
  await page.getByLabel('Country or territory').fill('日本 / Japan');
  await page.getByLabel('Additional details').fill('Ages 8–10, 60 students. <script>alert(1)</script>');
  const sent = [];
  page.on('request', request => { if (request.method() !== 'GET') sent.push(request.url()); });
  await review.click();
  const draft = page.locator('.draft-preview');
  await expect(draft).toContainText('東京国際学校 & Arts');
  await expect(draft).toContainText('+81 (0)3 1234-5678 ext. 12');
  await expect(draft).toContainText('東京都新宿区');
  await expect(draft).toContainText('日本 / Japan');
  await expect(draft).toContainText('<script>alert(1)</script>');
  await expect(draft.locator('script')).toHaveCount(0);
  await expect(draft).toContainText('not been sent or saved');
  await expect(page.locator('form a[href^="mailto:"]')).toHaveCount(0);
  await expect(page).toHaveURL(/booking.html\?program=anatomy$/);
  expect(sent).toEqual([]);
  expect(await page.evaluate(() => ({ local: localStorage.length, session: sessionStorage.length }))).toEqual({ local: 0, session: 0 });
  await page.getByLabel('Administrator or teacher name', { exact: true }).fill('Amira Hassan');
  await expect(draft).toHaveCount(0);
  await page.getByLabel('Country or territory').fill('الإمارات العربية المتحدة');
  await page.getByLabel('School address').fill('شارع المدرسة، دبي');
  await page.getByLabel('Phone number').fill('+971 50 123 4567');
  await page.getByLabel('Additional details').fill('');
  await review.click();
  await expect(draft).toContainText('الإمارات العربية المتحدة');
  await expect(draft).toContainText('Not provided');
});

test('school visit information is readable on the page without the PDF', async ({ page }) => {
  await page.goto('planning.html');
  await expect(page.locator('.faq')).toContainText('15 minutes');
  await page.locator('summary').filter({ hasText: 'Room setup & materials' }).click();
  await expect(page.getByText('White rolled paper and wall space for large drawings.', { exact: true })).toBeVisible();
  await page.locator('summary').filter({ hasText: 'International school visits' }).click();
  await expect(page.getByText(/Neal travels to schools around the world/)).toBeVisible();
  await expect(page.locator('a[href$=".pdf"]')).toHaveCount(0);
  await page.getByRole('link', { name: 'Start an inquiry' }).click();
  await expect(page.getByLabel('School name', { exact: true })).toBeVisible();
});

test('mobile navigation closes after selection and layouts do not overflow', async ({ page }) => {
  test.setTimeout(60000);
  for (const slug of ['index', 'programs', 'anatomy', 'geography-photos', 'anatomy-photos', 'art-photos', 'planning', 'booking', 'challenge', 'meet-neal', 'in-action', 'references']) {
  await page.goto(`${slug}.html`);
  await expect(page.locator('main h1')).toBeVisible();
  for (const width of [320, 390, 768, 1024, 1440]) {
    await page.setViewportSize({ width, height: 900 });
    const dimensions = await page.evaluate(() => ({ viewport: document.documentElement.clientWidth, content: document.documentElement.scrollWidth }));
    expect(dimensions.content, `${slug}: horizontal overflow at ${width}px`).toBeLessThanOrEqual(dimensions.viewport);
  }
  }
  await page.setViewportSize({ width: 390, height: 844 });
  const menu = page.getByRole('button', { name: 'Menu +' });
  await menu.click();
  await expect(page.locator('#main-nav')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Close −' })).toHaveAttribute('aria-expanded', 'true');
  await page.locator('#main-nav').getByRole('link', { name: 'Meet Neal' }).click();
  await expect(page.locator('#main-nav')).toBeHidden();
  await expect(page).toHaveURL(/meet-neal.html$/);
  await expect(page.locator('main h1')).toContainText('Meet Neal');
});

test('references are text-only, deduplicated, and grouped by source type', async ({ page }) => {
  await page.locator('#main-nav').getByRole('link', { name: 'References', exact: true }).click();
  await expect(page).toHaveURL(/references.html$/);
  await expect(page.locator('main h1')).toHaveText('References & press.');
  await expect(page.locator('main img, main iframe, main a[href^="http"]')).toHaveCount(0);
  await expect(page.locator('.reference-entry')).toHaveCount(6);
  await expect(page.locator('.directory-caption')).toHaveText('Showing 1–6 of 38');
  await expect(page.getByRole('heading', { name: 'Sea Girt Elementary School', exact: true })).toHaveCount(1);
  const westbrook = page.locator('.reference-entry').filter({ has: page.getByRole('heading', { name: 'Westbrook Hay Prep School', exact: true }) });
  await expect(westbrook.locator('blockquote')).toBeHidden();
  await westbrook.locator('summary').focus();
  await page.keyboard.press('Enter');
  await expect(westbrook.locator('blockquote')).toBeVisible();
  await expect(westbrook.locator('blockquote')).toContainText('It was a dynamic and fascinating way to learn');
  const cushing = page.locator('.reference-entry').filter({ has: page.getByRole('heading', { name: 'Cushing School PTO', exact: true }) });
  await cushing.locator('summary').click();
  await expect(cushing.locator('blockquote')).toContainText('The students were engaged the ENTIRE time.');
  await expect(cushing.locator('blockquote')).toBeVisible();
  await expect(westbrook.locator('blockquote')).toBeHidden();
  await expect(page).toHaveURL(/references.html$/);
  await page.getByRole('navigation', { name: 'Reference pages' }).getByRole('button', { name: 'Next' }).click();
  await expect(page.getByRole('heading', { name: 'CIS', exact: true })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'South', exact: true })).toBeVisible();
  const filters = page.getByRole('group', { name: 'Browse references' });
  await filters.getByRole('button', { name: /Community/ }).click();
  await expect(page.locator('.reference-entry')).toHaveCount(6);
  await expect(page.locator('.directory-caption')).toHaveText('Showing 1–6 of 11');
  await expect(page.getByRole('heading', { name: 'Wellfleet Rec', exact: true })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Jefferey Clayton', exact: true })).toBeVisible();
  await expect(page.locator('main a[href^="http"]')).toHaveCount(0);
  const capeCod = page.locator('.reference-entry').filter({ hasText: 'Friends of Cape Cod National Seashore' });
  await capeCod.locator('summary').click();
  await expect(capeCod.locator('blockquote')).toContainText('fun, entertaining and educational');
  const press = filters.getByRole('button', { name: /In the press/ });
  await press.focus();
  await page.keyboard.press('Enter');
  await expect(press).toHaveAttribute('aria-pressed', 'true');
  await expect(page.locator('.reference-entry')).toHaveCount(6);
  await expect(page.getByRole('heading', { name: 'Cook Islands News', exact: true })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Digo Dristi', exact: true })).toBeVisible();
  await expect(page.locator('main a[href^="http"], main iframe')).toHaveCount(0);
  const nome = page.locator('.reference-entry').filter({ has: page.getByRole('heading', { name: 'The Nome Nugget', exact: true }) });
  await expect(nome.locator('.coverage-text')).toBeHidden();
  await nome.locator('summary').click();
  await expect(nome.locator('.coverage-text')).toBeVisible();
  await expect(nome.locator('.coverage-text')).toContainText('September 23, 2010');
  await expect(nome.locator('.review-label')).toHaveText('Coverage summary');
  await expect(page).toHaveURL(/references.html$/);
  await expect(page.locator('main')).not.toContainText('School Instagram');
  await page.setViewportSize({ width: 320, height: 740 });
  await expect(nome.locator('.coverage-text')).toBeVisible();
  const dimensions = await page.evaluate(() => ({ content: document.documentElement.scrollWidth, viewport: innerWidth }));
  expect(dimensions.content).toBeLessThanOrEqual(dimensions.viewport);
  await nome.locator('summary').click();
  await expect(nome.locator('.coverage-text')).toBeHidden();
});

test('press and school articles expand into readable panels with searchable copy', async ({ page }) => {
  await page.goto('references.html');
  const search = page.getByRole('searchbox', { name: 'Find a reference' });
  for (const [publisher, count] of [['The Provincetown Independent', 2], ['The Nome Nugget', 1], ['Overdrive', 1], ['University of Delaware', 1], ['Peacehaven Community School', 2], ['Lancing Prep Hove', 3], ['James Hargest College', 1], ['Mrs. Thomas’s Fourth Grade', 1], ['Mrs. Begin’s Happy Campers', 1], ['Mike Mahaffie', 1]]) {
    await search.fill(publisher);
    const entry = page.locator('.reference-entry').filter({ has: page.getByRole('heading', {name: publisher, exact: true}) });
    await entry.locator('summary').click();
    await expect(entry.locator('.article-reading')).toHaveCount(count);
    await expect(entry.locator('.article-reading h4').first()).toBeVisible();
    expect((await entry.locator('.article-paragraph').allTextContents()).join(' ').split(/\s+/).length).toBeGreaterThan(45);
    expect((await entry.boundingBox()).width).toBeGreaterThan((await page.locator('.reference-list').boundingBox()).width * .95);
  }
  await search.fill('Eastham studio');
  await expect(page.locator('.reference-entry h3')).toHaveText('The Provincetown Independent');
  await page.locator('.reference-entry summary').click();
  await expect(page.locator('.article-reading h4')).toHaveText(['For Illustrator Neal Nichols Jr., Drawing Is Like Breathing', 'A Life That’s Everywhere and All Right Here']);
  await expect(page.locator('main a[href^="http"], main img, main iframe')).toHaveCount(0);
  await page.setViewportSize({width:320, height:740});
  await expect(page.locator('.article-reading').last()).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(320);
});

test('reference search and pagination retain the collection and consolidate online research', async ({ page }) => {
  await page.goto('references.html');
  const pages = page.getByRole('navigation', { name: 'Reference pages' });
  await expect(pages.getByRole('button', { name: 'Previous' })).toBeDisabled();
  const names = [];
  for (let index = 0; index < 7; index++) {
    await expect(pages).toContainText(`Page ${index + 1} of 7`);
    names.push(...await page.locator('.reference-entry h3').allTextContents());
    if (index < 6) await pages.getByRole('button', { name: 'Next' }).click();
  }
  expect(names).toHaveLength(38);
  expect(new Set(names).size).toBe(38);
  expect(names).toEqual(expect.arrayContaining(['CIS', 'South', 'Cushing School PTO', 'The Nome Nugget', 'James Hargest College', 'The Provincetown Independent', 'Lancing Prep Hove', 'Cape Cod National Seashore', 'K12 Academics']));
  await expect(pages.getByRole('button', { name: 'Next' })).toBeDisabled();

  const search = page.getByRole('searchbox', { name: 'Find a reference' });
  await search.fill('  lAnCiNg  ');
  await expect(page.locator('.reference-entry')).toHaveCount(1);
  await expect(page.getByRole('heading', { name: 'Lancing Prep Hove', exact: true })).toBeVisible();
  await expect(pages).toBeHidden();
  await page.locator('.reference-entry summary').click();
  await expect(page.locator('.coverage-text')).toContainText('2016–2017');
  await expect(page.locator('.coverage-text')).toContainText('December 20, 2024');

  const filters = page.getByRole('group', { name: 'Browse references' });
  await filters.getByRole('button', { name: /In the press/ }).click();
  await expect(page.getByRole('heading', { name: 'No references found.' })).toBeVisible();
  await page.getByRole('button', { name: 'Show all references', exact: true }).click();
  await expect(search).toHaveValue('');
  await expect(page.locator('.directory-caption')).toHaveText('Showing 1–6 of 38');

  await search.fill('New Zealand');
  await expect(page.locator('.reference-entry')).toHaveCount(2);
  await expect(page.getByRole('heading', { name: 'James Hargest College', exact: true })).toBeVisible();
  await page.getByRole('button', { name: 'Clear', exact: true }).click();
  await filters.getByRole('button', { name: /Schools & education/ }).click();
  await expect(page.locator('.directory-caption')).toHaveText('Showing 1–6 of 21');
  await search.fill('nonexistent-school-987654321');
  await expect(page.locator('.reference-entry')).toHaveCount(0);
  await expect(page.locator('.directory-caption')).toHaveText('No matching references');
  await page.setViewportSize({ width: 320, height: 740 });
  await page.getByRole('button', { name: 'Show all references', exact: true }).click();
  const dimensions = await page.evaluate(() => ({ content: document.documentElement.scrollWidth, viewport: innerWidth }));
  expect(dimensions.content).toBeLessThanOrEqual(dimensions.viewport);
});

test('ocean motion runs, pauses, resumes, and respects reduced motion', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'no-preference' });
  const ocean = page.locator('.ocean-texture');
  await expect(ocean).toHaveCSS('animation-play-state', 'running');
  const initial = await ocean.evaluate(el => getComputedStyle(el).transform);
  await expect.poll(() => ocean.evaluate(el => getComputedStyle(el).transform)).not.toBe(initial);
  await page.getByRole('button', { name: 'Pause ocean animation' }).click();
  await expect(ocean).toHaveCSS('animation-play-state', 'paused');
  await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  const paused = await ocean.evaluate(el => Array.from(new DOMMatrixReadOnly(getComputedStyle(el).transform).toFloat64Array()));
  await page.waitForTimeout(250); // Verify that elapsed time no longer advances the water.
  const settled = await ocean.evaluate(el => Array.from(new DOMMatrixReadOnly(getComputedStyle(el).transform).toFloat64Array()));
  // Browsers may finish a compositor frame after the pause. Subpixel rounding is harmless.
  expect(Math.max(...settled.map((value, i) => Math.abs(value - paused[i])))).toBeLessThan(0.05);
  await page.goto('programs.html');
  await expect(ocean).toHaveCSS('animation-play-state', 'paused');
  await page.getByRole('button', { name: 'Resume ocean animation' }).click();
  await expect(ocean).toHaveCSS('animation-play-state', 'running');
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await expect(ocean).toHaveCSS('animation-name', 'none');
  await expect(page.locator('.motion-toggle')).toBeHidden();
});

test('coastal opening pans, pauses, skips, replays, and remembers completion', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'no-preference' });
  const coast = page.locator('.coastal-home');
  const pan = page.locator('.coastal-pan');
  await expect(coast).toHaveClass(/coast-ready/);
  await expect(pan).toHaveCSS('animation-name', 'wellfleet-arrival');
  // The camera must keep advancing through the former horizon/harbor/water holds.
  const positions = await pan.evaluate(async el => {
    const animation = el.getAnimations()[0];
    const duration = animation.effect.getTiming().duration;
    const travel = el.getBoundingClientRect().height - innerHeight;
    const samples = [];
    for (const fraction of [.02, .12, .28, .34, .4, .62, .69, .75, .9, .98]) {
      animation.currentTime = duration * fraction;
      await new Promise(resolve => requestAnimationFrame(resolve));
      samples.push({ fraction, progress: -new DOMMatrixReadOnly(getComputedStyle(el).transform).m42 / travel });
    }
    animation.currentTime = 0;
    return samples;
  });
  for (const {fraction, progress} of positions) expect(Math.abs(progress - fraction)).toBeLessThan(.005);
  const start = await pan.evaluate(el => getComputedStyle(el).transform);
  await pan.evaluate(el => { el.getAnimations()[0].currentTime = 7500; });
  await expect.poll(() => pan.evaluate(el => getComputedStyle(el).transform)).not.toBe(start);

  await page.getByRole('button', { name: 'Pause ocean animation' }).click();
  await expect(pan).toHaveCSS('animation-play-state', 'paused');
  const pausedTime = await pan.evaluate(el => el.getAnimations()[0].currentTime);
  await page.waitForTimeout(150);
  const afterPause = await pan.evaluate(el => el.getAnimations()[0].currentTime);
  expect(Math.abs(afterPause - pausedTime)).toBeLessThan(40);
  await page.getByRole('button', { name: 'Resume ocean animation' }).click();
  await expect(pan).toHaveCSS('animation-play-state', 'running');

  await page.getByRole('button', { name: 'Skip intro' }).click();
  await expect(coast).toHaveClass(/coast-done/);
  await expect(page.locator('.coastal-content')).toHaveCSS('opacity', '1');
  await expect(page.getByRole('link', { name: /Explore the programs/ })).toBeVisible();
  expect(await page.evaluate(() => sessionStorage.getItem('wellfleet-arrival-seen'))).toBe('true');
  await page.goto('programs.html');
  await expect(page.locator('.coastal-pan')).toHaveCount(0);
  await page.goto('index.html');
  await expect(coast).toHaveClass(/coast-done/);
  await page.getByRole('button', { name: 'Replay coast' }).click();
  await expect(coast).not.toHaveClass(/coast-done/);
  await expect(pan).toHaveCSS('animation-name', 'wellfleet-arrival');
  await pan.evaluate(el => el.getAnimations()[0].finish());
  await expect(coast).toHaveClass(/coast-done/);
  await expect(page.locator('.coastal-content')).toHaveCSS('opacity', '1');
});

test('coastal opening respects reduced motion and keeps keyboard access immediate', async ({ page }) => {
  await expect(page.locator('.coastal-pan')).toHaveCSS('animation-name', 'none');
  await expect(page.locator('.coastal-content')).toHaveCSS('opacity', '1');
  await expect(page.locator('.coastal-controls')).toBeHidden();
  await page.emulateMedia({ reducedMotion: 'no-preference' });
  await page.getByRole('link', { name: /Explore the programs/ }).focus();
  await expect(page.locator('.coastal-home')).toHaveClass(/coast-done/);
  await expect(page.locator('.coastal-content')).toHaveCSS('opacity', '1');
});
