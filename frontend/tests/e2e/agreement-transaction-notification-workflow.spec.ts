import { expect, test } from '@playwright/test';

test('agreement rule creates transaction after milestone approval and notification trail appears', async ({
  page,
}) => {
  let payoutRules: any[] = [];
  let milestones: any[] = [];
  let transactions: any[] = [];
  let notifications: any[] = [];
  await page.addInitScript(() => {
    window.localStorage.setItem(
      'auth:user',
      JSON.stringify({
        id: 1,
        email: 'admin@atlas.test',
        name: 'Avery Atlas',
        user_type: 'admin',
        role: 'admin',
        portal: 'admin',
      }),
    );

    window.localStorage.setItem('auth:token', 'e2e-admin-token');
  });

  await page.route('**/api/engagements/1', async (route) => {
    await route.fulfill({
      json: {
        id: 1,
        organization_id: 1,
        project_id: 1,
        client_id: 1,
        title: 'Agreement Transaction Workflow',
        status: 'active',
      },
    });
  });

  await page.route('**/api/organizations/1/parties', async (route) => {
    await route.fulfill({
      json: [
        {
          id: 1,
          organization_id: 1,
          name: 'Riverbend Municipal Water Authority',
          email: 'ops@riverbend.gov',
          party_type: 'client',
          is_verified: 1,
        },
        {
          id: 2,
          organization_id: 1,
          name: 'Avery Atlas',
          email: 'admin@atlas.test',
          party_type: 'contractor',
          is_verified: 1,
        },
      ],
    });
  });

  await page.route('**/api/parties/*/payment-readiness', async (route) => {
    const partyId = Number(
      route
        .request()
        .url()
        .match(/parties\/(\d+)/)?.[1],
    );

    await route.fulfill({
      json: {
        is_verified: true,
        payer_ready: partyId === 1,
        payee_ready: partyId === 2,
        payment_profile: {
          party_id: partyId,
          payment_role: partyId === 1 ? 'payer' : 'payee',
          payer_authorization_status:
            partyId === 1 ? 'authorized' : 'not_configured',
          payout_status: partyId === 2 ? 'ready' : 'not_ready',
        },
      },
    });
  });

  await page.route('**/api/organizations/1/agreements', async (route) => {
    await route.fulfill({
      json: [
        {
          id: 1,
          organization_id: 1,
          engagement_id: 1,
          title: 'Client pays contractor on milestone approval',
          agreement_type: 'milestone_payout',
          status: 'draft',
        },
      ],
    });
  });

  await page.route('**/api/agreements/1/payout-rules', async (route) => {
    if (route.request().method() === 'POST') {
      const body = route.request().postDataJSON();

      const rule = {
        id: 1,
        agreement_id: 1,
        from_party_id: Number(body.from_party_id),
        to_party_id: Number(body.to_party_id),
        rule_type: body.rule_type,
        percent: Number(body.percent),
        amount_cents: body.amount_cents,
        trigger_event: body.trigger_event,
      };

      payoutRules = [rule];

      await route.fulfill({ json: rule });
      return;
    }

    await route.fulfill({ json: payoutRules });
  });

  await page.route('**/api/engagements/1/milestones', async (route) => {
    if (route.request().method() === 'POST') {
      const body = route.request().postDataJSON();

      const milestone = {
        id: 1,
        engagement_id: 1,
        title: body.title,
        description: body.description,
        amount_cents: Number(body.amount_cents ?? 2500),
        status: 'submitted',
      };

      milestones = [milestone];

      await route.fulfill({ json: milestone });
      return;
    }

    await route.fulfill({ json: milestones });
  });

  await page.route('**/api/milestones/*/approve', async (route) => {
    milestones = milestones.map((milestone) => ({
      ...milestone,
      status: 'approved',
    }));

    transactions = [
      {
        id: 1,
        engagement_id: 1,
        agreement_id: 1,
        from_party_id: 1,
        to_party_id: 2,
        amount_cents: 2500,
        currency: 'usd',
        status: 'pending',
        trigger_event: 'MilestoneApproved',
      },
    ];

    await route.fulfill({
      json: milestones[0],
    });
  });

  await page.route('**/api/engagements/1/transactions', async (route) => {
    await route.fulfill({
      json: transactions,
    });
  });
  await page.route('**/api/notifications', async (route) => {
    await route.fulfill({
      json: notifications,
    });
  });
  await page.route('**/api/transactions/*/mark-paid', async (route) => {
    const transactionId = Number(
      route
        .request()
        .url()
        .match(/transactions\/(\d+)/)?.[1],
    );

    transactions = transactions.map((transaction) =>
      transaction.id === transactionId
        ? {
            ...transaction,
            status: 'paid',
          }
        : transaction,
    );

    notifications = [
      {
        id: 1,
        title: 'Transaction marked paid',
        body: 'A payout transaction was marked paid for Agreement Transaction Workflow.',
        notification_type: 'transaction_paid',
        read_at: null,
        created_at: '2026-05-20 00:00:00',
      },
    ];

    await route.fulfill({
      json: transactions.find(
        (transaction) => transaction.id === transactionId,
      ),
    });
  });

  await page.route(
    '**/api/operational-transactions/*/mark-paid',
    async (route) => {
      const transactionId = Number(
        route
          .request()
          .url()
          .match(/operational-transactions\/(\d+)/)?.[1],
      );

      transactions = transactions.map((transaction) =>
        transaction.id === transactionId
          ? {
              ...transaction,
              status: 'paid',
            }
          : transaction,
      );

      notifications = [
        {
          id: 1,
          title: 'Transaction marked paid',
          body: 'A payout transaction was marked paid for Agreement Transaction Workflow.',
          notification_type: 'transaction_paid',
          read_at: null,
          created_at: '2026-05-20 00:00:00',
        },
      ];

      await route.fulfill({
        json: transactions.find(
          (transaction) => transaction.id === transactionId,
        ),
      });
    },
  );
  await page.goto('/engagements/1/agreements');

  await expect(page.getByText('Agreement Rules')).toBeVisible();

  await page
    .locator('.ops-card')
    .filter({ hasText: 'Riverbend Municipal Water Authority' })
    .getByRole('button', { name: /prepare as payer/i })
    .click();

  await page
    .locator('.ops-card')
    .filter({ hasText: 'Avery Atlas' })
    .getByRole('button', { name: /prepare as payee/i })
    .click();

  await expect(page.getByText(/✓ Verified payer selected/i)).toBeVisible();
  await expect(page.getByText(/✓ Payer funding authorized/i)).toBeVisible();
  await expect(page.getByText(/✓ Verified payee selected/i)).toBeVisible();
  await expect(page.getByText(/✓ Payee payout-ready/i)).toBeVisible();

  await page.locator('#payer-party').selectOption('1');
  await page.locator('#payee-party').selectOption('2');

  await expect(
    page.getByRole('button', { name: /add payout rule/i }),
  ).toBeEnabled();

  await page.getByRole('button', { name: /add payout rule/i }).click();

  await expect(page.getByText(/contractor_payout/i)).toBeVisible();

  await page.goto('/engagements/1/milestones');
  await page
    .getByRole('button', { name: /add milestone|create milestone/i })
    .click();
  const milestoneTitle = `E2E Milestone ${Date.now()}`;

  await page.locator('input').first().fill(milestoneTitle);

  const amountInput = page.locator('input[type="number"]').first();

  if (await amountInput.count()) {
    await amountInput.fill('2500');
  }

  const descriptionInput = page.locator('textarea').first();

  if (await descriptionInput.count()) {
    await descriptionInput.fill(
      'Milestone approval should generate payout transaction.',
    );
  }

  await page
    .getByRole('button', { name: /add milestone|create milestone/i })
    .click();

  await expect(page.getByText(milestoneTitle)).toBeVisible();

  await page
    .locator('.ops-card, .milestone-card, .portal-section')
    .filter({ hasText: milestoneTitle })
    .getByRole('button', { name: /approve/i })
    .click();

  await expect(page.getByText(/approved/i).first()).toBeVisible();

  await page.goto('/engagements/1/transactions');

  await expect(
    page.getByRole('heading', { name: /operational transactions/i }),
  ).toBeVisible();

  await expect(page.getByText(/pending/i).first()).toBeVisible();

  await expect(page.getByText(/\$25\.00|\$25/i).first()).toBeVisible();

  await expect(page.getByText(/MilestoneApproved/i).first()).toBeVisible();
  const paidButton = page
    .getByRole('button', { name: /paid|mark as paid|mark paid/i })
    .first();

  await expect(paidButton).toBeVisible();
  await paidButton.click();

  await expect(page.getByText(/paid/i).first()).toBeVisible();

  await page.goto('/notifications');

  await expect(
    page.getByRole('heading', { name: /notifications/i }),
  ).toBeVisible();

  await expect(page.getByText(/transaction marked paid/i)).toBeVisible();
  await expect(page.getByText(/transaction_paid/i)).toBeVisible();
});
