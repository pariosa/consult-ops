import { test, expect } from '@playwright/test';

test('super admin can manage organizations users and memberships', async ({
  page,
}) => {
  let organizations = [
    {
      id: 1,
      name: 'Atlas Field Consulting',
      slug: 'atlas-field-consulting',
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
      'auth_user',
      JSON.stringify({
        id: 1,
        email: 'superadmin@consultops.test',
        name: 'Platform Super Admin',
        user_type: 'super_admin',
        role: 'super_admin',
        portal: 'platform',
      }),
    );

    window.localStorage.setItem('auth_token', 'e2e-super-admin-token');
  });

  await page.route('**/api/platform/organizations', async (route) => {
    if (route.request().method() === 'POST') {
      const body = route.request().postDataJSON();

      const org = {
        id: 2,
        name: body.name,
        slug: body.name.toLowerCase().replaceAll(' ', '-'),
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
    const match = route
      .request()
      .url()
      .match(/organizations\/(\d+)\/members/);
    const organizationId = Number(match?.[1] ?? 1);

    if (route.request().method() === 'POST') {
      const body = route.request().postDataJSON();
      const user = users.find((item) => item.id === Number(body.user_id));

      const member = {
        id: 1,
        organization_id: organizationId,
        user_id: Number(body.user_id),
        email: user?.email,
        name: user?.name,
        user_type: user?.user_type,
        role: body.role,
        status: 'active',
        created_at: '2026-05-15',
        updated_at: '2026-05-15',
      };

      members = [member];

      await route.fulfill({ json: member });
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

  await page
    .locator('#platform-assignment-organization')
    .selectOption({ label: 'New Rescue Org' });

  await page.locator('#platform-assignment-user').selectOption('2');

  await page.locator('#platform-assignment-role').selectOption('admin');

  await page.getByRole('button', { name: /assign user/i }).click();

  await expect(page.getByText(/user assigned to organization/i)).toBeVisible();

  await expect(
    page.locator('.table-row').filter({ hasText: 'new.admin@example.com' }),
  ).toContainText('active');
});

test('non super admin is blocked from platform admin UI', async ({ page }) => {
  await page.addInitScript(() => {
    window.localStorage.setItem(
      'auth_user',
      JSON.stringify({
        id: 2,
        email: 'admin@atlas.test',
        name: 'Atlas Admin',
        user_type: 'admin',
        role: 'admin',
      }),
    );

    window.localStorage.setItem('auth_token', 'e2e-admin-token');
  });

  await page.goto('/platform');

  const unauthorizedHeading = page.getByRole('heading', {
    name: /you do not have permission/i,
  });

  const platformHeading = page.getByRole('heading', {
    name: /platform admin/i,
  });

  await expect(unauthorizedHeading.or(platformHeading)).toBeVisible();

  if (await platformHeading.isVisible().catch(() => false)) {
    await expect(
      page.getByText(/platform admin access required/i),
    ).toBeVisible();

    await expect(
      page.getByRole('button', { name: /create organization/i }),
    ).toHaveCount(0);
  } else {
    await expect(unauthorizedHeading).toBeVisible();
  }
});
