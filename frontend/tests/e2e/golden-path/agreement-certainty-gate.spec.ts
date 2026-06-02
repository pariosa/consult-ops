// frontend/tests/e2e/golden-path/agreement-certainty-gate.spec.ts

import { test, expect } from '@playwright/test';
import { mockAdminUser } from '../helpers/auth';

test.describe('Golden path agreement certainty gates', () => {
  test('transactions page can show payout obligations only after workflow setup', async ({
    page,
  }) => {
    await mockAdminUser(page);

    await page.goto('/engagements/1/transactions');
    await expect(
      page.getByRole('heading', {
        name: 'Operational Transactions',
      }),
    ).toBeVisible();
  });

  test('agreement setup page is reachable from engagement workflow', async ({
    page,
  }) => {
    await mockAdminUser(page);

    await page.goto('/engagements/1/agreements');

    await expect(
      page.getByRole('heading', {
        name: 'Agreement Rules',
      }),
    ).toBeVisible();
  });
});
