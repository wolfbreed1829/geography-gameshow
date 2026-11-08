import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  workers: 2,
  expect: { timeout: 15000 },
  reporter: 'list',
  use: {
    baseURL: process.env.PREVIEW_URL || 'http://127.0.0.1:8080',
    channel: process.env.PLAYWRIGHT_CHANNEL || undefined,
    reducedMotion: 'reduce',
    viewport: { width: 1440, height: 1000 },
  },
});
