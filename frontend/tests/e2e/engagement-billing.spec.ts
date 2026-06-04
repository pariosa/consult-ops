import { test, expect } from '@playwright/test';

test('billing page shows activation payment state', async ({ page }) => {
  await page.route('**/api/engagements/6', async (route) => {
    await route.fulfill({
      json: {
        id: 6,
        title: 'Paid Activation Flow',
        contractor_name: 'Peter Ariosa',
        contractor_email: '[ariosa@gmail.com](mailto:ariosa@gmail.com)',
        scope_of_work: 'Validate payment-driven activation.',
        status: 'awaiting_payment',
        platform_fee_status: 'pending',
      },
    });
  });

  await page.route('**/api/engagements/6/milestones', async (route) => {
    await route.fulfill({ json: [] });
  });

  await page.route('**/api/engagements/6/events', async (route) => {
    await route.fulfill({ json: [] });
  });

  await page.route('**/api/engagements/6/billing', async (route) => {
    await route.fulfill({
      json: [
        {
          id: 30,
          engagement_id: 6,
          organization_id: 1,
          billing_type: 'activation_fee',
          amount_cents: 1000,
          currency: 'usd',
          status: 'pending',
          stripe_checkout_session_id: 'cs_test_123',
          stripe_payment_intent_id: null,
          paid_at: null,
          created_at: '2026-05-13 10:55:00',
        },
      ],
    });
  });

  await page.goto('/engagements/6');

  await expect(
    page.getByText(/Activation payment is the next operational gate/i),
  ).toBeVisible();

  await expect(page.getByText(/Activation Pending/i)).toBeVisible();
  await page.goto('/engagements/6/billing');
  await expect(
    page.getByRole('heading', { name: /activation payment/i }),
  ).toBeVisible();
  await expect(page.getByText('Activation Fee', { exact: true })).toBeVisible();
  await expect(
    page.getByRole('button', { name: /Pay Activation Fee/i }),
  ).toBeVisible();
});
