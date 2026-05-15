import { test, expect } from '@playwright/test';

test('dev mark paid updates billing and shows activation events', async ({ page }) => {
  let billingStatus = 'pending';
  let engagementStatus = 'awaiting_payment';
  let events: any[] = [];

  await page.route('**/api/engagements/6', async route => {
    await route.fulfill({
      json: {
        id: 6,
        title: 'Paid Activation Flow',
        contractor_name: 'Peter Ariosa',
        contractor_email: 'ariosa@gmail.com',
        scope_of_work: 'Validate payment-driven activation.',
        status: engagementStatus,
        platform_fee_status: billingStatus,
      },
    });
  });

  await page.route('**/api/engagements/6/milestones', async route => {
    await route.fulfill({ json: [] });
  });

  await page.route('**/api/engagements/6/events', async route => {
    await route.fulfill({ json: events });
  });

  await page.route('**/api/engagements/6/billing', async route => {
    await route.fulfill({
      json: [
        {
          id: 30,
          engagement_id: 6,
          organization_id: 1,
          billing_type: 'activation_fee',
          amount_cents: 1000,
          currency: 'usd',
          status: billingStatus,
          stripe_checkout_session_id: 'cs_test_123',
          stripe_payment_intent_id: null,
          paid_at: billingStatus === 'paid' ? '2026-05-13 11:00:00' : null,
          created_at: '2026-05-13 10:55:00',
        },
      ],
    });
  });

  await page.route('**/api/engagement-billing/30/mark-paid', async route => {
    billingStatus = 'paid';
    engagementStatus = 'active';

    events = [
      {
        id: 1,
        event_type: 'ActivationFeePaid',
        from_status: 'pending',
        to_status: 'paid',
        created_at: '2026-05-13 11:00:00',
      },
      {
        id: 2,
        event_type: 'PaymentReceived',
        from_status: 'awaiting_payment',
        to_status: 'active',
        created_at: '2026-05-13 11:00:01',
      },
    ];

    await route.fulfill({
      json: {
        id: 30,
        engagement_id: 6,
        organization_id: 1,
        billing_type: 'activation_fee',
        amount_cents: 1000,
        currency: 'usd',
        status: 'paid',
        stripe_checkout_session_id: 'cs_test_123',
        stripe_payment_intent_id: null,
        paid_at: '2026-05-13 11:00:00',
        created_at: '2026-05-13 10:55:00',
      },
    });
  });

  await page.goto('/engagements/6');

  await expect(page.getByText('Status: pending')).toBeVisible();

  await page.getByRole('button', { name: /dev: mark paid/i }).click();

  await expect(page.getByText('Status: paid')).toBeVisible();
  await expect(page.getByText(/Activation Fee Paid/i)).toBeVisible();
});