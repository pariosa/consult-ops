// frontend/tests/e2e/onboarding.spec.ts

import { test, expect } from '@playwright/test';
import {
  mockNoOrgUser,
  mockSingleOrgUser,
  mockMultiOrgUser,
} from './helpers/auth';

test.describe('Onboarding routing', () => {
  test('user with no organization routes to onboarding', async ({ page }) => {
    await mockNoOrgUser(page);

    await page.goto('/');

    await expect(page).toHaveURL(/\/onboarding/);
  });

  test('user with multiple organizations routes to workspace-select', async ({
    page,
  }) => {
    await mockMultiOrgUser(page);

    await page.goto('/');

    await expect(page).toHaveURL(/\/workspace-select/);
  });

  test('user with one organization routes to project-portal', async ({
    page,
  }) => {
    await mockSingleOrgUser(page);

    await page.goto('/');

    await expect(page).toHaveURL(/\/project-portal/);
  });
});
