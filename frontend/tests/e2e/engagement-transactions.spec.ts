import { test, expect } from '@playwright/test';

test('engagement transactions page renders generated payout obligations', async ({
  page,
}) => {
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
          status: 'pending',
          trigger_event: 'MilestoneApproved',
          created_at: '2026-05-15 04:56:01',
        },
        {
          id: 2,
          organization_id: 1,
          agreement_id: 1,
          engagement_id: 1,
          milestone_id: 1,
          from_party_id: 2,
          to_party_id: 3,
          transaction_type: 'subcontractor_payout',
          amount_cents: 600,
          currency: 'usd',
          status: 'paid',
          trigger_event: 'MilestoneApproved',
          created_at: '2026-05-15 04:56:01',
        },
      ],
    });
  });

  await page.goto('/engagements/1/transactions');

  await expect(page.getByText('Operational Transactions')).toBeVisible();
  await expect(
    page.getByText('contractor payout', { exact: true }),
  ).toBeVisible();
  await expect(
    page.getByText('subcontractor payout', { exact: true }),
  ).toBeVisible();
  await expect(page.getByText('$20.00')).toBeVisible();
  await expect(page.getByText('$6.00')).toBeVisible();
  await expect(page.getByText('pending')).toBeVisible();
  await expect(page.getByText('paid')).toBeVisible();
});

test('engagement transactions page renders empty state', async ({ page }) => {
  await page.route('**/api/engagements/2/transactions', async (route) => {
    await route.fulfill({ json: [] });
  });

  await page.goto('/engagements/2/transactions');

  await expect(
    page.getByText('No operational transactions have been generated yet.'),
  ).toBeVisible();
});
