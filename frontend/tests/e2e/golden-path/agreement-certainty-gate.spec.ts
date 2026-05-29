// frontend/tests/e2e/golden-path/agreement-certainty-gate.spec.ts
import { test, expect } from '@playwright/test';
import { loginAsAdmin } from '../helpers/auth';

test.describe('Golden path agreement certainty gates', () => {
  test('milestone payment is blocked when no operational agreement exists', async ({
    page,
  }) => {
    await loginAsAdmin(page);

    await page.goto('/engagements/1/milestones');

    await page.getByRole('button', { name: /approve milestone/i }).click();

    await expect(
      page.getByText(/agreement required|operational agreement required/i),
    ).toBeVisible();

    await expect(
      page.getByRole('button', { name: /mark payment sent/i }),
    ).toBeDisabled();

    await page
      .getByRole('link', { name: /set up agreement|agreement/i })
      .click();

    await expect(page).toHaveURL(/\/engagements\/1\/agreements/);
  });

  test('milestone payment is blocked when agreement exists but payout rules are missing', async ({
    page,
  }) => {
    await loginAsAdmin(page);

    await page.goto('/engagements/1/agreements');

    await page
      .getByRole('button', { name: /create operational agreement/i })
      .click();

    await page.goto('/engagements/1/milestones');

    await page.getByRole('button', { name: /approve milestone/i }).click();

    await expect(
      page.getByText(/payout rules required|configure payout rules/i),
    ).toBeVisible();

    await expect(
      page.getByRole('button', { name: /mark payment sent/i }),
    ).toBeDisabled();

    await page
      .getByRole('link', { name: /configure payout rules|agreement/i })
      .click();

    await expect(page).toHaveURL(/\/engagements\/1\/agreements/);
  });
});
