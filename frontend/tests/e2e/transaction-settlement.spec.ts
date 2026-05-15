import { test, expect } from '@playwright/test';

test('user can mark a pending transaction as processing and then paid', async ({
  page,
}) => {
  let status = 'pending';

  await page.route('**/api/engagements/1/transactions', async (route) => {
    await route.fulfill({
      json: [
        {
          id: 1,
          organization_id: 1,
          agreement_id: 1,
          engagement_id: 1,
          milestone_id: 1,
          from_party_id: 1,
          to_party_id: 2,
          transaction_type: 'contractor_payout',
          amount_cents: 2000,
          currency: 'usd',
          status,
          trigger_event: 'MilestoneApproved',
          created_at: '2026-05-15 04:56:01',
        },
      ],
    });
  });

  await page.route(
    '**/api/operational-transactions/1/mark-processing',
    async (route) => {
      status = 'processing';

      await route.fulfill({
        json: {
          id: 1,
          status: 'processing',
        },
      });
    },
  );

  await page.route(
    '**/api/operational-transactions/1/mark-paid',
    async (route) => {
      status = 'paid';

      await route.fulfill({
        json: {
          id: 1,
          status: 'paid',
        },
      });
    },
  );

  await page.goto('/engagements/1/transactions');

  await expect(page.getByText('pending', { exact: true })).toBeVisible();

  await page.getByRole('button', { name: /^processing$/i }).click();

  await expect(page.getByText('processing', { exact: true })).toBeVisible();

  await page.getByRole('button', { name: /^paid$/i }).click();

  await expect(page.getByText('paid', { exact: true })).toBeVisible();
  await expect(page.getByRole('button', { name: /^processing$/i })).toHaveCount(
    0,
  );
});
test('user can cancel a pending transaction', async ({ page }) => {
  let status = 'pending';

  await page.route('**/api/engagements/2/transactions', async (route) => {
    await route.fulfill({
      json: [
        {
          id: 2,
          organization_id: 1,
          agreement_id: 1,
          engagement_id: 2,
          milestone_id: 1,
          from_party_id: 1,
          to_party_id: 2,
          transaction_type: 'subcontractor_payout',
          amount_cents: 600,
          currency: 'usd',
          status,
          trigger_event: 'MilestoneApproved',
          created_at: '2026-05-15 04:56:01',
        },
      ],
    });
  });

  await page.route(
    '**/api/operational-transactions/2/cancel',
    async (route) => {
      status = 'cancelled';

      await route.fulfill({
        json: {
          id: 2,
          status: 'cancelled',
        },
      });
    },
  );

  await page.goto('/engagements/2/transactions');

  await expect(page.getByText('pending', { exact: true })).toBeVisible();

  await page.getByRole('button', { name: /^cancel$/i }).click();

  await expect(page.getByText('cancelled', { exact: true })).toBeVisible();
});
