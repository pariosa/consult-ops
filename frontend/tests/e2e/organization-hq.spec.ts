import { test, expect } from '@playwright/test';

test('admin sees role-aware organization workspace navigation', async ({
  page,
}) => {
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
    page.getByRole('heading', { name: 'Organization HQ' }),
  ).toBeVisible();
  await expect(page.getByText('Atlas Operations')).toBeVisible();
  await expect(
    page.locator('.role-pill').filter({ hasText: /^admin$/ }),
  ).toBeVisible();
  await expect(
    page.getByRole('link', { name: 'Members', exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole('link', { name: 'Invitations', exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole('link', { name: 'Operational Finance', exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole('link', { name: 'Projects', exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole('link', { name: 'Clients', exact: true }),
  ).toBeVisible();
  await expect(page.getByText('Agreements').first()).toBeVisible();
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
        user_type: 'contractor',
      }),
    );
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
    page.getByRole('heading', { name: 'Organization HQ' }),
  ).toBeVisible();
  await expect(page.getByText('Atlas Operations')).toBeVisible();
  await expect(
    page.locator('.role-pill').filter({ hasText: /^contractor$/ }),
  ).toBeVisible();
  await expect(
    page.getByRole('link', { name: 'Projects', exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole('link', { name: 'Clients', exact: true }),
  ).toBeVisible();

  await expect(
    page.getByRole('link', { name: 'Operational Finance', exact: true }),
  ).toHaveCount(0);
  await expect(
    page.getByRole('link', { name: 'Invitations', exact: true }),
  ).toHaveCount(0);
});
