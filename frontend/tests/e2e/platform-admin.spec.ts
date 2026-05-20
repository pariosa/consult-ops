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
        name: 'Platform Super Admin',
        user_type: 'super_admin',
        role: 'super_admin',
        portal: 'platform',
      }),
    );
    window.localStorage.setItem('auth:token', 'e2e-super-admin-token');
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
          organization_id: Number(body.organization_id ?? 1),
          user_id: Number(body.user_id),
          email: user?.email,
          name: user?.name,
          user_type: user?.user_type,
          role: body.role,
          status: 'active',
          created_at: '2026-05-15',
          updated_at: '2026-05-15',
        },
      ];

      await route.fulfill({ json: { success: true } });
      return;
    }

    await route.fulfill({ json: members });
  });

  await page.goto('/platform');

  await expect(
    page.getByRole('heading', { name: /platform admin/i }),
  ).toBeVisible();

  await expect(page.getByText(/platform admin access required/i)).toHaveCount(
    0,
  );

  await page.getByLabel(/organization name/i).fill('New Rescue Org');
  await page.getByRole('button', { name: /create organization/i }).click();

  await expect(page.getByText(/organization created/i)).toBeVisible();

  await page.locator('#platform-user-email').fill('new.admin@example.com');
  await page.locator('#platform-user-name').fill('New Admin');
  await page.locator('#platform-user-type').selectOption('admin');
  await page.locator('#platform-user-password').fill('DemoPass123!');
  await page.getByRole('button', { name: /create user/i }).click();

  await expect(page.getByText(/user created/i)).toBeVisible();

  await page.locator('select').nth(1).selectOption('1');
  await page.locator('select').nth(2).selectOption('2');
  await page.locator('select').nth(3).selectOption('admin');
  await page.getByRole('button', { name: /assign user/i }).click();

  await expect(page.getByText(/user assigned to organization/i)).toBeVisible();
  await expect(
    page.locator('.table-row').filter({ hasText: 'new.admin@example.com' }),
  ).toContainText('active');
  await expect(page.getByText('active')).toBeVisible();
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

  await expect(
    page.getByRole('heading', { name: /platform admin/i }),
  ).toBeVisible();

  await expect(page.getByText(/platform admin access required/i)).toBeVisible();
  await expect(
    page.getByRole('button', { name: 'Create Organization' }),
  ).toHaveCount(0);
});
