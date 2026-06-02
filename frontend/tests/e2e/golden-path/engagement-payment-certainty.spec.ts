// frontend/tests/e2e/golden-path/engagement-payment-certainty.spec.ts

import { test, expect } from '@playwright/test';

test('golden path: milestone approval creates transaction and audit trail', async ({
  page,
}) => {
  let milestoneStatus = 'pending';

  let events: any[] = [
    {
      id: 1,
      event_type: 'AgreementLocked',
      from_status: null,
      to_status: 'locked',
      created_at: '2026-05-13 09:00:00',
    },
    {
      id: 2,
      event_type: 'PayoutRuleConfigured',
      from_status: null,
      to_status: 'configured',
      created_at: '2026-05-13 09:05:00',
    },
  ];

  let transactions: any[] = [];

  const transaction = {
    id: 900,
    organization_id: 1,
    agreement_id: 300,
    engagement_id: 77,
    milestone_id: 700,
    transaction_type: 'milestone_payout',
    from_party_id: 100,
    to_party_id: 200,
    amount_cents: 120000,
    currency: 'usd',
    status: 'pending',
    trigger_event: 'milestone_approved',
    created_at: '2026-05-13 10:05:00',
    updated_at: '2026-05-13 10:05:00',
  };

  await page.route('**/api/engagements/77', async (route) => {
    await route.fulfill({
      json: {
        id: 77,
        title: 'Golden Path Engagement',
        contractor_name: 'Golden Contractor',
        contractor_email: 'contractor@example.com',
        scope_of_work: 'Deliver the approved software milestone.',
        status: 'active',
        platform_fee_status: 'paid',
        amount_cents: 120000,
      },
    });
  });

  await page.route('**/api/engagements/77/billing', async (route) => {
    await route.fulfill({
      json: [
        {
          id: 10,
          engagement_id: 77,
          organization_id: 1,
          billing_type: 'activation_fee',
          amount_cents: 1000,
          currency: 'usd',
          status: 'paid',
          created_at: '2026-05-13 09:00:00',
        },
      ],
    });
  });

  await page.route('**/api/engagements/77/events', async (route) => {
    await route.fulfill({ json: events });
  });

  await page.route('**/api/engagements/77/milestones', async (route) => {
    await route.fulfill({
      json: [
        {
          id: 700,
          engagement_id: 77,
          title: 'Homepage Delivery',
          description: 'Deliver homepage implementation.',
          amount_cents: 120000,
          due_date: '2026-05-30',
          status: milestoneStatus,
          created_at: '2026-05-13 09:10:00',
        },
      ],
    });
  });

  await page.route('**/api/milestones/700/submit', async (route) => {
    milestoneStatus = 'submitted';

    events = [
      {
        id: 3,
        event_type: 'MilestoneSubmitted',
        from_status: 'pending',
        to_status: 'submitted',
        created_at: '2026-05-13 10:00:00',
      },
      ...events,
    ];

    await route.fulfill({
      json: {
        id: 700,
        engagement_id: 77,
        title: 'Homepage Delivery',
        amount_cents: 120000,
        status: 'submitted',
      },
    });
  });

  await page.route('**/api/milestones/700/approve', async (route) => {
    milestoneStatus = 'approved';
    transactions = [transaction];

    events = [
      {
        id: 4,
        event_type: 'MilestoneApproved',
        from_status: 'submitted',
        to_status: 'approved',
        created_at: '2026-05-13 10:05:00',
      },
      {
        id: 5,
        event_type: 'OperationalTransactionCreated',
        from_status: null,
        to_status: 'pending',
        created_at: '2026-05-13 10:06:00',
      },
      ...events,
    ];

    await route.fulfill({
      json: {
        id: 700,
        engagement_id: 77,
        title: 'Homepage Delivery',
        amount_cents: 120000,
        status: 'approved',
      },
    });
  });

  await page.route('**/api/engagements/77/transactions', async (route) => {
    await route.fulfill({ json: transactions });
  });

  await page.goto('/engagements/77/milestones');

  await expect(page.getByText(/Homepage Delivery/i)).toBeVisible();

  const milestoneCard = page
    .locator('.milestone-card')
    .filter({ hasText: /Homepage Delivery/i })
    .first();

  await expect(milestoneCard).toBeVisible();

  await milestoneCard.getByTestId('submit-milestone-button').click();
  await expect(milestoneCard).toContainText(/submitted/i);

  const submittedCard = page
    .locator('.milestone-card')
    .filter({ hasText: /Homepage Delivery/i })
    .filter({ hasText: /submitted/i })
    .first();

  await expect(submittedCard).toBeVisible();

  await submittedCard.getByTestId('approve-milestone-button').click();

  const approvedCard = page
    .locator('.milestone-card')
    .filter({ hasText: /Homepage Delivery/i })
    .filter({ hasText: /approved/i })
    .first();

  await expect(approvedCard).toBeVisible();

  await page.goto('/engagements/77/transactions');

  await expect(
    page.getByRole('heading', { name: /operational transactions/i }),
  ).toBeVisible();

  await expect(page.getByText(/milestone payout/i)).toBeVisible();
  await expect(page.getByText(/#100/i)).toBeVisible();
  await expect(page.getByText(/#200/i)).toBeVisible();
  await expect(page.getByText(/\$1,200\.00/i)).toBeVisible();
  await expect(page.getByText(/pending/i)).toBeVisible();
  await expect(page.getByText(/milestone_approved/i)).toBeVisible();

  await page.goto('/engagements/77');

  await expect(
    page.getByText(/AgreementLocked|Agreement Locked/i),
  ).toBeVisible();

  await expect(
    page.getByText(/PayoutRuleConfigured|Payout Rule/i),
  ).toBeVisible();

  await expect(
    page.getByText(/MilestoneApproved|Milestone Approved/i),
  ).toBeVisible();

  await expect(
    page.getByText(/OperationalTransactionCreated|Transaction Created/i),
  ).toBeVisible();
});
