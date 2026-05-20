import { expect, test } from '@playwright/test';
import { mockAuthUser } from './helpers/auth';

test('admin can create, prepare, verify, configure, and authorize payer/payee', async ({
  page,
}) => {
  let parties: any[] = [];
  let profiles: Record<number, any> = {};

  await mockAuthUser(page, {
    id: 1,
    email: 'superadmin@consultops.test',
    name: 'Platform Super Admin',
    user_type: 'super_admin',
    role: 'super_admin',
    portal: 'platform',
  });
  await page.route('**/api/engagements/1', async (route) => {
    await route.fulfill({
      json: {
        id: 1,
        organization_id: 1,
        project_id: 1,
        title: 'E2E Agreement Engagement',
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
    await route.fulfill({ json: [] });
  });

  await page.route('**/api/organizations/1/parties', async (route) => {
    if (route.request().method() === 'POST') {
      const body = route.request().postDataJSON();
      const party = {
        id: parties.length + 1,
        organization_id: 1,
        name: body.name,
        email: body.email,
        party_type: body.party_type,
        is_verified: 0,
        verification_status: 'unverified',
      };
      parties = [party, ...parties];
      await route.fulfill({ json: party });
      return;
    }

    await route.fulfill({ json: parties });
  });

  await page.route('**/api/parties/*/verify', async (route) => {
    const partyId = Number(
      route
        .request()
        .url()
        .match(/parties\/(\d+)/)?.[1],
    );
    parties = parties.map((party) =>
      party.id === partyId
        ? {
            ...party,
            is_verified: 1,
            verification_status: 'verified',
            verification_method: 'admin',
          }
        : party,
    );
    await route.fulfill({
      json: parties.find((party) => party.id === partyId),
    });
  });

  await page.route('**/api/parties/*/payment-profile', async (route) => {
    const partyId = Number(
      route
        .request()
        .url()
        .match(/parties\/(\d+)/)?.[1],
    );
    const body = route.request().postDataJSON();

    profiles[partyId] = {
      id: partyId,
      party_id: partyId,
      organization_id: 1,
      payment_role: body.payment_role,
      payer_authorization_status: 'not_configured',
      payer_authorization_scope: body.payer_authorization_scope ?? null,
      payout_status: 'not_ready',
      stripe_connect_onboarding_status: 'not_started',
    };

    await route.fulfill({ json: profiles[partyId] });
  });

  await page.route('**/api/parties/*/payer-authorized/dev', async (route) => {
    const partyId = Number(
      route
        .request()
        .url()
        .match(/parties\/(\d+)/)?.[1],
    );
    profiles[partyId] = {
      ...(profiles[partyId] ?? {}),
      party_id: partyId,
      payment_role: 'payer',
      payer_authorization_status: 'authorized',
      stripe_customer_id: `cus_dev_party_${partyId}`,
      stripe_payment_method_id: `pm_dev_party_${partyId}`,
      payout_status: profiles[partyId]?.payout_status ?? 'not_ready',
    };
    await route.fulfill({ json: profiles[partyId] });
  });

  await page.route('**/api/parties/*/payout-ready/dev', async (route) => {
    const partyId = Number(
      route
        .request()
        .url()
        .match(/parties\/(\d+)/)?.[1],
    );
    profiles[partyId] = {
      ...(profiles[partyId] ?? {}),
      party_id: partyId,
      payment_role: 'payee',
      payout_status: 'ready',
      stripe_connect_onboarding_status: 'complete',
      stripe_connect_account_id: `acct_dev_party_${partyId}`,
      payer_authorization_status:
        profiles[partyId]?.payer_authorization_status ?? 'not_configured',
    };
    await route.fulfill({ json: profiles[partyId] });
  });

  await page.route('**/api/parties/*/payment-readiness', async (route) => {
    const partyId = Number(
      route
        .request()
        .url()
        .match(/parties\/(\d+)/)?.[1],
    );
    const party = parties.find((item) => item.id === partyId);
    const profile = profiles[partyId] ?? null;

    await route.fulfill({
      json: {
        party,
        payment_profile: profile,
        is_verified: Number(party?.is_verified) === 1,
        payer_ready:
          profile?.payment_role === 'payer' &&
          profile?.payer_authorization_status === 'authorized',
        payee_ready:
          profile?.payment_role === 'payee' &&
          profile?.payout_status === 'ready',
      },
    });
  });

  await page.goto('/engagements/1/agreements');

  await expect(
    page.getByRole('heading', { name: /agreement rules/i }),
  ).toBeVisible();

  const timestamp = Date.now();
  const clientName = `E2E Client ${timestamp}`;
  const clientEmail = `client-${timestamp}@example.com`;
  const contractorName = `E2E Contractor ${timestamp}`;
  const contractorEmail = `contractor-${timestamp}@example.com`;

  const clientCard = page.locator('.portal-section').filter({
    has: page.getByRole('heading', { name: /create client/i }),
  });

  await expect(
    page.getByText(
      /only admins, operations managers, finance admins, or payment moderators/i,
    ),
  ).toHaveCount(0);

  await expect(
    page.getByRole('heading', { name: /create client/i }),
  ).toBeVisible();

  await clientCard.locator('input').nth(0).fill(clientName);
  await clientCard.locator('input').nth(1).fill(clientEmail);
  await clientCard.getByRole('button', { name: /add client party/i }).click();

  const contractorCard = page.locator('.portal-section').filter({
    has: page.getByRole('heading', { name: /create contractor/i }),
  });

  await contractorCard.locator('input').nth(0).fill(contractorName);
  await contractorCard.locator('input').nth(1).fill(contractorEmail);
  await contractorCard
    .getByRole('button', { name: /add contractor party/i })
    .click();

  await expect(page.getByText(clientName).first()).toBeVisible();
  await expect(page.getByText(contractorName).first()).toBeVisible();

  await page
    .locator('.ops-card')
    .filter({ hasText: clientName })
    .getByRole('button', { name: /prepare as payer/i })
    .click();

  await page
    .locator('.ops-card')
    .filter({ hasText: contractorName })
    .getByRole('button', { name: /prepare as payee/i })
    .click();

  await page.getByRole('button', { name: /verify payer/i }).click();
  await page.getByRole('button', { name: /configure as payer/i }).click();
  await page.getByRole('button', { name: /authorize funding dev/i }).click();

  await page.getByRole('button', { name: /verify payee/i }).click();
  await page.getByRole('button', { name: /configure as payee/i }).click();
  await page.getByRole('button', { name: /mark payout ready dev/i }).click();

  await expect(page.getByText(/✓ Verified payer selected/i)).toBeVisible();
  await expect(page.getByText(/✓ Payer funding authorized/i)).toBeVisible();
  await expect(page.getByText(/✓ Verified payee selected/i)).toBeVisible();
  await expect(page.getByText(/✓ Payee payout-ready/i)).toBeVisible();

  await expect(page.locator('#payer-party')).toContainText(clientName);
  await expect(page.locator('#payee-party')).toContainText(contractorName);
});
