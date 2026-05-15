import { test, expect } from '@playwright/test';

test('admin can configure agreement with verified parties', async ({
  page,
}) => {
  await page.route('**/api/engagements/1', async (route) => {
    await route.fulfill({
      json: {
        id: 1,
        organization_id: 1,
        client_id: 1,
        title: 'Verified Agreement Flow',
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
        {
          id: 3,
          organization_id: 1,
          name: 'Unverified Vendor',
          email: 'vendor@example.com',
          party_type: 'contractor',
          is_verified: 0,
        },
      ],
    });
  });
  await page.route('**/api/organizations/1/agreements', async (route) => {
    if (route.request().method() === 'POST') {
      await route.fulfill({
        json: {
          id: 1,
          organization_id: 1,
          engagement_id: 1,
          title: 'Client pays contractor on milestone approval',
          agreement_type: 'milestone_payout',
          status: 'draft',
        },
      });
      return;
    }

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
      await route.fulfill({
        json: {
          id: 1,
          agreement_id: 1,
          from_party_id: 1,
          to_party_id: 2,
          rule_type: 'contractor_payout',
          percent: 100,
          amount_cents: null,
          trigger_event: 'MilestoneApproved',
        },
      });
      return;
    }

    await route.fulfill({
      json: [],
    });
  });

  await page.addInitScript(() => {
    window.localStorage.setItem(
      'auth:user',
      JSON.stringify({
        id: 1,
        email: 'admin@atlas.test',
        user_type: 'admin',
      }),
    );
  });
  await page.goto('/engagements/1/agreements');

  await expect(page.getByText('Agreement Rules')).toBeVisible();

  await expect(page.getByText('Unverified Vendor')).toHaveCount(0);
  const payer = page.locator('#payer-party');
  const payee = page.locator('#payee-party');

  await expect(payer.locator('option[value="1"]')).toHaveText(
    'Riverbend Municipal Water Authority — Verified client',
  );

  await expect(payee.locator('option[value="2"]')).toHaveText(
    'Avery Atlas — Verified contractor',
  );

  await payer.selectOption('1');
  await payee.selectOption('2');

  await expect(payer).toHaveValue('1');
  await expect(payee).toHaveValue('2');

  await expect(page.locator('#payer-party')).toHaveValue('1');
  await expect(page.locator('#payee-party')).toHaveValue('2');
});
test('unverified parties are not selectable for core payout rules', async ({
  page,
}) => {
  await page.route('**/api/engagements/1', async (route) => {
    await route.fulfill({
      json: {
        id: 1,
        organization_id: 1,
        client_id: 1,
        title: 'Verified Agreement Flow',
      },
    });
  });

  await page.route('**/api/organizations/1/parties', async (route) => {
    await route.fulfill({
      json: [
        {
          id: 99,
          organization_id: 1,
          name: 'Unverified Vendor',
          email: 'vendor@example.com',
          party_type: 'contractor',
          is_verified: 0,
        },
      ],
    });
  });

  await page.route('**/api/organizations/1/agreements', async (route) => {
    await route.fulfill({ json: [] });
  });

  await page.goto('/engagements/1/agreements');

  await expect(page.getByText('Unverified Vendor')).toHaveCount(0);
});
