// frontend/tests/e2e/agreement-locking.spec.ts
import { expect, test } from '@playwright/test';

test('admin can lock an agreement after payout rule setup and rules become protected', async ({
  page,
}) => {
  let agreement = {
    id: 1,
    organization_id: 1,
    engagement_id: 1,
    title: 'Client pays contractor on milestone approval',
    agreement_type: 'milestone_payout',
    status: 'draft',
  };

  let payoutRules: any[] = [];

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
        title: 'Agreement Locking Workflow',
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
          name: 'Clientey Clintor',
          email: 'client@client.com',
          party_type: 'client',
          is_verified: 1,
        },
        {
          id: 2,
          organization_id: 1,
          name: 'contractor bobhbee',
          email: 'contractor@bobby.com',
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
    await route.fulfill({ json: [agreement] });
  });

  await page.route('**/api/agreements/1/payout-rules', async (route) => {
    if (route.request().method() === 'POST') {
      if (agreement.status === 'locked') {
        await route.fulfill({
          status: 409,
          body: 'Agreement is locked and cannot be modified.',
        });
        return;
      }

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

  await page.route('**/api/agreements/1/lock', async (route) => {
    agreement = {
      ...agreement,
      status: 'locked',
    };

    await route.fulfill({ json: agreement });
  });

  await page.goto('/engagements/1/agreements');

  await expect(page.getByText(/agreement rules/i)).toBeVisible();

  await page
    .locator('.ops-card')
    .filter({ hasText: 'Clientey Clintor' })
    .getByRole('button', { name: /prepare as payer/i })
    .click();

  await page
    .locator('.ops-card')
    .filter({ hasText: 'contractor bobhbee' })
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

  await expect(
    page.getByRole('button', { name: /lock agreement/i }),
  ).toBeEnabled();

  await page.getByRole('button', { name: /lock agreement/i }).click();

  await expect(page.getByText(/status:\s*locked/i)).toBeVisible();

  await expect(
    page.getByText(/agreement locked\. payout rules are protected/i),
  ).toBeVisible();

  await expect(page.locator('#payer-party')).toBeDisabled();
  await expect(page.locator('#payee-party')).toBeDisabled();

  await expect(
    page.getByRole('button', { name: /add payout rule/i }),
  ).toBeDisabled();
});
