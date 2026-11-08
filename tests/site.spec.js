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
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  await page.reload();
  await page.getByRole('heading', { name: 'Big ideas. Drawn live.' }).waitFor();
  const assets = await page.evaluate(async () => {
    document.querySelectorAll('img').forEach(img => img.loading = 'eager');
    await Promise.all([...document.images].map(img => img.decode()));
    await document.fonts.ready;
    const ocean = new Image();
    ocean.src = new URL('public/ocean-background.png', document.baseURI).href;
    await ocean.decode();
    return { images: [...document.images].every(img => img.naturalWidth > 0), fonts: [...document.fonts].every(font => font.status === 'loaded'), ocean: ocean.naturalWidth > 0 };
  });
  expect(assets).toEqual({ images: true, fonts: true, ocean: true });
  expect(errors).toEqual([]);
});

test('program links and keyboard tabs select the correct details', async ({ page }) => {
  await page.locator('.biology-card').click();
  await expect(page.getByRole('tabpanel')).toContainText('BIOLOGY / CONCEPT PREVIEW');
  const biology = page.getByRole('tab', { name: 'Biology' });
  await expect(biology).toHaveAttribute('aria-selected', 'true');
  await biology.focus();
  await page.keyboard.press('ArrowRight');
  await expect(page.getByRole('tab', { name: 'Art', exact: true })).toBeFocused();
  await expect(page.getByRole('tabpanel')).toContainText('ART / CONCEPT PREVIEW');
  await page.keyboard.press('Home');
  await expect(page.getByRole('tab', { name: 'Geography' })).toBeFocused();
  await expect(page.getByRole('tabpanel')).toContainText('THE ORIGINAL LIVE EXPERIENCE');
});

for (const [topic, answers] of [['Geography', [1, 0, 2]], ['Biology', [0, 1, 2]], ['Art', [1, 0, 2]]]) {
  test(`${topic} challenge scores correctly, prevents duplicate answers, and resets`, async ({ page }) => {
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
    await quiz.getByRole('button', { name: topic === 'Art' ? 'Biology' : 'Art', exact: true }).click();
    await expect(quiz.locator('.quiz-progress')).toContainText('QUESTION 01 / 03');
    await expect(quiz.locator('.quiz-answers button:disabled')).toHaveCount(0);
  });
}

test('inquiry validates, safely encodes a draft, and clears stale drafts on edits', async ({ page }) => {
  await page.getByRole('button', { name: 'Prepare my inquiry' }).click();
  await expect(page.locator('.draft-preview')).toHaveCount(0);
  await page.getByLabel('Your name', { exact: true }).fill('Alex & Sam');
  await page.getByLabel('Email address', { exact: true }).fill('alex+school@example.org');
  await page.getByLabel('School or organization', { exact: true }).fill('Science & Art School');
  await page.getByLabel('What are you curious about?').selectOption('the Biology concept');
  await page.getByLabel('A little about your group').fill('Grades 3–4, 60 students.\nBoston & nearby. Dates to discuss.');
  await page.getByRole('button', { name: 'Prepare my inquiry' }).click();
  const href = await page.getByRole('link', { name: 'Open in my email app' }).getAttribute('href');
  const url = new URL(href);
  expect(url.protocol).toBe('mailto:');
  expect(url.pathname).toBe('geographygameshow@yahoo.com');
  expect(url.searchParams.get('subject')).toBe('Show inquiry: the Biology concept');
  expect(url.searchParams.get('body')).toContain('Alex & Sam');
  expect(url.searchParams.get('body')).toContain('Boston & nearby.');
  expect(url.searchParams.get('body')).toContain('alex+school@example.org');
  await page.getByLabel('Your name', { exact: true }).fill('Updated name');
  await expect(page.locator('.draft-preview')).toHaveCount(0);
});

test('mobile navigation closes after selection and layouts do not overflow', async ({ page }) => {
  for (const width of [320, 390, 768, 1024, 1440]) {
    await page.setViewportSize({ width, height: 900 });
    const dimensions = await page.evaluate(() => ({ viewport: document.documentElement.clientWidth, content: document.documentElement.scrollWidth }));
    expect(dimensions.content, `horizontal overflow at ${width}px`).toBeLessThanOrEqual(dimensions.viewport);
  }
  await page.setViewportSize({ width: 390, height: 844 });
  const menu = page.getByRole('button', { name: 'Menu +' });
  await menu.click();
  await expect(page.locator('#main-nav')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Close −' })).toHaveAttribute('aria-expanded', 'true');
  await page.locator('#main-nav').getByRole('link', { name: 'Meet Neal' }).click();
  await expect(page.locator('#main-nav')).toBeHidden();
  await expect(page).toHaveURL(/#meet-neal$/);
});

test('ocean motion runs, pauses, resumes, and respects reduced motion', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'no-preference' });
  const ocean = page.locator('.ocean-texture');
  await expect(ocean).toHaveCSS('animation-play-state', 'running');
  const initial = await ocean.evaluate(el => getComputedStyle(el).transform);
  await expect.poll(() => ocean.evaluate(el => getComputedStyle(el).transform)).not.toBe(initial);
  await page.getByRole('button', { name: 'Pause ocean animation' }).click();
  await expect(ocean).toHaveCSS('animation-play-state', 'paused');
  const paused = await ocean.evaluate(el => getComputedStyle(el).transform);
  await page.waitForTimeout(250); // Verify that elapsed time no longer advances the water.
  expect(await ocean.evaluate(el => getComputedStyle(el).transform)).toBe(paused);
  await page.getByRole('button', { name: 'Resume ocean animation' }).click();
  await expect(ocean).toHaveCSS('animation-play-state', 'running');
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await expect(ocean).toHaveCSS('animation-name', 'none');
  await expect(page.locator('.motion-toggle')).toBeHidden();
});
