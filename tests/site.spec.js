import { test, expect } from '@playwright/test';

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
  await expect(page.locator('.show-card, .quiz-box, .inquiry-form')).toHaveCount(0);
  expect(await page.evaluate(() => document.documentElement.scrollHeight)).toBeLessThanOrEqual(1050);
  await page.locator('#main-nav').getByRole('link', { name: 'Programs', exact: true }).click();
  await expect(page).toHaveURL(/programs.html$/);
  await expect(page.locator('.show-card')).toHaveCount(3);
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
  for (const slug of ['index', 'programs', 'geography', 'anatomy', 'art', 'in-action', 'meet-neal', 'challenge', 'planning', 'booking']) {
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
  for (const slug of ['index', 'programs', 'anatomy', 'planning', 'booking', 'challenge', 'meet-neal', 'in-action']) {
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
