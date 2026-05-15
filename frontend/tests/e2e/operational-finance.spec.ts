import { test, expect } from '@playwright/test';

test('organization finance dashboard renders summary and party balances', async ({
  page,
}) => {
  await page.route('**/api/me/organization', async (route) => {
    await route.fulfill({
      json: {
        id: 1,
        name: 'Atlas Operations',
      },
    });
  });

  await page.route('**/api/organizations/1/finance-summary', async (route) => {
    await route.fulfill({
      json: {
        organization_id: 1,
        pending_cents: 2000,
        processing_cents: 600,
        paid_cents: 5000,
        failed_cents: 0,
        cancelled_cents: 0,
        total_obligations_cents: 7600,
      },
    });
  });

  await page.route('**/api/organizations/1/party-balances', async (route) => {
    await route.fulfill({
      json: [
        {
          party_id: 1,
          party_name: 'Riverbend Municipal Water Authority',
          party_type: 'client',
          is_verified: 1,
          payable_cents: 2600,
          receivable_cents: 0,
          net_cents: -2600,
        },
        {
          party_id: 2,
          party_name: 'Avery Atlas',
          party_type: 'contractor',
          is_verified: 1,
          payable_cents: 0,
          receivable_cents: 2600,
          net_cents: 2600,
        },
      ],
    });
  });

  await page.goto('/organization/finance');

  await expect(page.getByText('Operational Finance')).toBeVisible();
  await expect(page.getByText('$26.00').first()).toBeVisible();
  await expect(page.getByText('$50.00')).toBeVisible();
  await expect(
    page.getByText('Riverbend Municipal Water Authority'),
  ).toBeVisible();
  await expect(page.getByText('Avery Atlas')).toBeVisible();
});
