import { test, expect } from '@playwright/test';

test('engagement page shows operational timeline and activation billing', async ({
  page,
}) => {
  await page.route('**/api/engagements/6', async (route) => {
    await route.fulfill({
      json: {
        id: 6,
        title: 'Build DevOps Platform',
        contractor_name: 'Peter Ariosa',
        contractor_email: 'ariosa@gmail.com',
        scope_of_work: 'Build deployment automation and operational workflows.',
        status: 'awaiting_payment',
        platform_fee_status: 'pending',
      },
    });
  });

  await page.route('**/api/engagements/6/milestones', async (route) => {
    await route.fulfill({ json: [] });
  });

  await page.route('**/api/engagements/6/events', async (route) => {
    await route.fulfill({
      json: [
        {
          id: 1,
          event_type: 'EngagementCreated',
          from_status: null,
          to_status: 'draft',
          created_at: '2026-05-12 12:00:00',
        },
        {
          id: 2,
          event_type: 'ActivationFeeCreated',
          from_status: null,
          to_status: 'pending',
          created_at: '2026-05-12 12:05:00',
        },
      ],
    });
  });

  await page.route('**/api/engagements/6/billing', async (route) => {
    await route.fulfill({
      json: [
        {
          id: 10,
          billing_type: 'activation_fee',
          amount_cents: 1000,
          currency: 'usd',
          status: 'pending',
          stripe_checkout_session_id: null,
        },
      ],
    });
  });

  await page.goto('/engagements/6');

  await expect(page.getByText('Engagement Tracker')).toBeVisible();
  await expect(page.getByText('Build DevOps Platform')).toBeVisible();
  await expect(page.getByText('Operational History')).toBeVisible();
  await expect(page.getByText('Activation Billing')).toBeVisible();
  await expect(page.getByText('activation_fee')).toBeVisible();
});

test('pay activation fee redirects to Stripe checkout URL', async ({
  page,
}) => {
  await page.route('**/api/engagements/6', async (route) => {
    await route.fulfill({
      json: {
        id: 6,
        title: 'Build DevOps Platform',
        contractor_name: 'Peter Ariosa',
        contractor_email: 'ariosa@gmail.com',
        scope_of_work: 'Build deployment automation.',
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
    await route.fulfill({ json: [] });
  });

  await page.route(
    '**/api/engagements/6/activation-checkout',
    async (route) => {
      await route.fulfill({
        json: {
          url: 'https://checkout.stripe.com/test-session',
        },
      });
    },
  );

  await page.goto('/engagements/6');

  await page.getByRole('button', { name: /pay activation fee/i }).click();

  await expect(page).toHaveURL(/checkout\.stripe\.com/);
});
