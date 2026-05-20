import { expect, test } from '@playwright/test';

test('admin sees role-aware organization workspace navigation', async ({
  page,
}) => {
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

  await page.route('**/api/me/organization', async (route) => {
    await route.fulfill({
      json: {
        id: 1,
        name: 'Atlas Operations',
      },
    });
  });

  await page.goto('/organization');

  await expect(
    page.getByRole('heading', { name: /organization hq/i }),
  ).toBeVisible();

  await expect(page.getByText('Atlas Operations')).toBeVisible();

  await expect(
    page.getByRole('link', { name: 'Projects', exact: true }).first(),
  ).toBeVisible();

  await expect(
    page.getByRole('link', { name: 'Clients', exact: true }).first(),
  ).toBeVisible();

  await expect(
    page.getByRole('link', { name: 'Engagements', exact: true }).first(),
  ).toBeVisible();
});

test('contractor sees limited organization workspace navigation', async ({
  page,
}) => {
  await page.addInitScript(() => {
    window.localStorage.setItem(
      'auth:user',
      JSON.stringify({
        id: 2,
        email: 'contractor@example.com',
        name: 'Contractor User',
        user_type: 'contractor',
        role: 'contractor',
        portal: 'contractor',
      }),
    );
    window.localStorage.setItem('auth:token', 'e2e-contractor-token');
  });

  await page.route('**/api/me/organization', async (route) => {
    await route.fulfill({
      json: {
        id: 1,
        name: 'Atlas Operations',
      },
    });
  });

  await page.goto('/organization');

  await expect(
    page.getByRole('heading', { name: /organization hq/i }),
  ).toBeVisible();

  await expect(page.getByText('Atlas Operations')).toBeVisible();

  await expect(
    page.getByRole('link', { name: 'Projects', exact: true }).first(),
  ).toBeVisible();

  await expect(
    page.getByRole('link', { name: 'Clients', exact: true }).first(),
  ).toBeVisible();

  await expect(
    page.getByRole('link', { name: 'Operational Finance', exact: true }),
  ).toHaveCount(0);

  await expect(
    page.getByRole('link', { name: 'Invitations', exact: true }),
  ).toHaveCount(0);

  await expect(
    page.getByRole('link', { name: 'Users', exact: true }),
  ).toHaveCount(0);
});
