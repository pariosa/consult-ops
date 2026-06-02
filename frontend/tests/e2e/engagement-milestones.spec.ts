import { test, expect } from '@playwright/test';

test('milestone submit and approve actions refresh the operational timeline', async ({
  page,
}) => {
  let milestoneStatus = 'pending';
  let events: any[] = [
    {
      id: 1,
      event_type: 'EngagementCreated',
      from_status: null,
      to_status: 'draft',
      created_at: '2026-05-13 09:00:00',
    },
  ];

  await page.route('**/api/engagements/6', async (route) => {
    await route.fulfill({
      json: {
        id: 6,
        title: 'Build Consulting Ops Platform',
        contractor_name: 'Peter Ariosa',
        contractor_email: 'ariosa@gmail.com',
        scope_of_work:
          'Build contract, milestone, billing, and event workflows.',
        status: 'active',
        platform_fee_status: 'paid',
      },
    });
  });

  await page.route('**/api/engagements/6/milestones', async (route) => {
    await route.fulfill({
      json: [
        {
          id: 11,
          engagement_id: 6,
          title: 'Implement operational timeline',
          description:
            'Create auditable event history for engagement workflow.',
          amount_cents: 250000,
          due_date: '2026-05-20',
          status: milestoneStatus,
          created_at: '2026-05-13 09:10:00',
        },
      ],
    });
  });

  await page.route('**/api/engagements/6/events', async (route) => {
    await route.fulfill({ json: events });
  });

  await page.route('**/api/engagements/6/billing', async (route) => {
    await route.fulfill({
      json: [
        {
          id: 20,
          engagement_id: 6,
          organization_id: 1,
          billing_type: 'activation_fee',
          amount_cents: 1000,
          currency: 'usd',
          status: 'paid',
          stripe_checkout_session_id: 'cs_test_123',
          stripe_payment_intent_id: null,
          paid_at: '2026-05-13 09:15:00',
          created_at: '2026-05-13 09:12:00',
        },
      ],
    });
  });

  await page.route('**/api/milestones/11/submit', async (route) => {
    milestoneStatus = 'submitted';
    events = [
      {
        id: 2,
        event_type: 'MilestoneSubmitted',
        from_status: null,
        to_status: 'submitted',
        created_at: '2026-05-13 10:00:00',
      },
      ...events,
    ];

    await route.fulfill({
      json: {
        id: 11,
        engagement_id: 6,
        title: 'Implement operational timeline',
        description: 'Create auditable event history for engagement workflow.',
        amount_cents: 250000,
        due_date: '2026-05-20',
        status: 'submitted',
        created_at: '2026-05-13 09:10:00',
      },
    });
  });

  await page.route('**/api/milestones/11/approve', async (route) => {
    milestoneStatus = 'approved';
    events = [
      {
        id: 3,
        event_type: 'MilestoneApproved',
        from_status: null,
        to_status: 'approved',
        created_at: '2026-05-13 10:05:00',
      },
      ...events,
    ];

    await route.fulfill({
      json: {
        id: 11,
        engagement_id: 6,
        title: 'Implement operational timeline',
        description: 'Create auditable event history for engagement workflow.',
        amount_cents: 250000,
        due_date: '2026-05-20',
        status: 'approved',
        created_at: '2026-05-13 09:10:00',
      },
    });
  });

  await page.goto('/engagements/6/milestones');

  await expect(
    page.getByRole('heading', { name: /milestone management/i }),
  ).toBeVisible();

  await expect(page.getByText(/Implement operational timeline/i)).toBeVisible();

  const milestoneCard = page
    .locator('.milestone-card')
    .filter({ hasText: /Implement operational timeline/i })
    .first();

  await expect(milestoneCard).toBeVisible();

  await milestoneCard.getByTestId('submit-milestone-button').click();

  await expect(page.getByText(/submitted/i)).toBeVisible();
  await expect(
    page.getByText(/MilestoneSubmitted|Milestone submitted|submitted/i),
  ).toBeVisible();
  const submittedCard = page
    .locator('.milestone-card')
    .filter({ hasText: /Implement operational timeline/i })
    .filter({ hasText: /submitted/i })
    .first();

  await expect(submittedCard).toBeVisible();

  await submittedCard.getByTestId('approve-milestone-button').click();

  await expect(page.getByText(/approved/i)).toBeVisible();
  await expect(
    page.getByText(/MilestoneApproved|Milestone approved|approved/i),
  ).toBeVisible();
});
