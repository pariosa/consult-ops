import { test, expect } from '@playwright/test';

test('super admin can manage organizations users and memberships', async ({
  page,
}) => {
  let organizations = [
    {
      id: 1,
      name: 'Atlas Field Consulting',
      created_at: '2026-05-15',
      updated_at: '2026-05-15',
    },
  ];

  let users = [
    {
      id: 1,
      email: 'superadmin@consultops.test',
      name: 'Platform Super Admin',
      user_type: 'super_admin',
      created_at: '2026-05-15',
      updated_at: '2026-05-15',
    },
  ];

  let members: any[] = [];

  await page.addInitScript(() => {
    window.localStorage.setItem(
      'auth:user',
      JSON.stringify({
        id: 1,
        email: 'superadmin@consultops.test',
        user_type: 'super_admin',
      }),
    );
  });

  await page.route('**/api/platform/organizations', async (route) => {
    if (route.request().method() === 'POST') {
      const body = route.request().postDataJSON();

      const org = {
        id: 2,
        name: body.name,
        created_at: '2026-05-15',
        updated_at: '2026-05-15',
      };

      organizations = [org, ...organizations];

      await route.fulfill({ json: org });
      return;
    }

    await route.fulfill({ json: organizations });
  });

  await page.route('**/api/platform/users', async (route) => {
    if (route.request().method() === 'POST') {
      const body = route.request().postDataJSON();

      const user = {
        id: 2,
        email: body.email,
        name: body.name,
        user_type: body.user_type,
        created_at: '2026-05-15',
        updated_at: '2026-05-15',
      };

      users = [user, ...users];

      await route.fulfill({ json: user });
      return;
    }

    await route.fulfill({ json: users });
  });

  await page.route('**/api/platform/organizations/*/members', async (route) => {
    if (route.request().method() === 'POST') {
      const body = route.request().postDataJSON();
      const user = users.find((item) => item.id === Number(body.user_id));

      members = [
        {
          id: 1,
          organization_id: 1,
          user_id: body.user_id,
          email: user?.email,
          name: user?.name,
          user_type: user?.user_type,
          role: body.role,
          status: 'active',
          created_at: '2026-05-15',
          updated_at: '2026-05-15',
        },
      ];

      await route.fulfill({
        json: {
          success: true,
        },
      });
      return;
    }

    await route.fulfill({ json: members });
  });

  await page.goto('/platform');

  await expect(
    page.getByRole('heading', { name: 'Platform Admin' }),
  ).toBeVisible();

  await page.locator('#platform-org-name').fill('New Rescue Org');

  await page.getByRole('button', { name: 'Create Organization' }).click();

  await expect(page.getByText('Organization created.')).toBeVisible();

  await page.locator('#platform-user-email').fill('new.admin@example.com');
  await page.locator('#platform-user-name').fill('New Admin');
  await page.locator('#platform-user-type').selectOption('admin');
  await page.locator('#platform-user-password').fill('DemoPass123!');
  await page.getByRole('button', { name: 'Create User' }).click();

  await expect(page.getByText('User created.')).toBeVisible();

  await page.locator('#platform-assignment-organization').selectOption('1');
  await page.locator('#platform-assignment-user').selectOption('2');
  await page.locator('#platform-assignment-role').selectOption('admin');
  await page.getByRole('button', { name: 'Assign User' }).click();

  await expect(page.getByText('User assigned to organization.')).toBeVisible();
  await expect(
    page.locator('.table-row').filter({ hasText: 'new.admin@example.com' }),
  ).toBeVisible();
  await expect(page.getByText('admin').first()).toBeVisible();
  await expect(
    page.locator('.table-row').filter({ hasText: 'active' }),
  ).toBeVisible();
});

test('non super admin is blocked from platform admin UI', async ({ page }) => {
  await page.addInitScript(() => {
    window.localStorage.setItem(
      'auth:user',
      JSON.stringify({
        id: 2,
        email: 'admin@atlas.test',
        user_type: 'admin',
      }),
    );
  });

  await page.goto('/platform');

  await expect(page.getByText('Platform admin access required.')).toBeVisible();
  await expect(
    page.getByRole('button', { name: 'Create Organization' }),
  ).toHaveCount(0);
});
