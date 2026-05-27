import { test, expect } from '@playwright/test';

function seedSuperAdmin(page: any) {
  return page.addInitScript(() => {
    window.localStorage.setItem(
      'auth_user',
      JSON.stringify({
        id: 1,
        email: 'superadmin@consultops.test',
        name: 'Platform Super Admin',
        user_type: 'super_admin',
        role: 'super_admin',
        portal: 'platform',
        token: 'e2e-super-admin-token',
      }),
    );

    window.localStorage.setItem('auth_token', 'e2e-super-admin-token');
  });
}

function seedNormalAdmin(page: any) {
  return page.addInitScript(() => {
    window.localStorage.setItem(
      'auth_user',
      JSON.stringify({
        id: 2,
        email: 'admin@atlas.test',
        name: 'Atlas Admin',
        user_type: 'admin',
        role: 'admin',
        token: 'e2e-admin-token',
      }),
    );

    window.localStorage.setItem('auth_token', 'e2e-admin-token');
  });
}

test('super admin can create organization, create user, and assign membership', async ({
  page,
}) => {
  let organizations = [
    {
      id: 1,
      name: 'Atlas Field Consulting',
      slug: 'atlas-field-consulting',
    },
  ];

  let users = [
    {
      id: 1,
      email: 'superadmin@consultops.test',
      name: 'Platform Super Admin',
      user_type: 'super_admin',
    },
  ];

  let members: any[] = [];

  await seedSuperAdmin(page);

  await page.route('**/api/notifications', async (route) => {
    await route.fulfill({ json: [] });
  });

  await page.route('**/api/platform/organizations', async (route) => {
    if (route.request().method() === 'POST') {
      const body = route.request().postDataJSON();

      const org = {
        id: 2,
        name: body.name,
        slug: body.name.toLowerCase().replaceAll(' ', '-'),
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

test('super admin can use user detail administrative actions', async ({
  page,
}) => {
  await seedSuperAdmin(page);

  await page.route('**/api/notifications', async (route) => {
    await route.fulfill({ json: [] });
  });

  await page.route('**/api/admin/users/2', async (route) => {
    await route.fulfill({
      json: {
        id: 2,
        email: 'new.admin@example.com',
        name: 'New Admin',
        user_type: 'admin',
        disabled_at: null,
      },
    });
  });

  await page.route('**/api/admin/users/2/memberships', async (route) => {
    await route.fulfill({
      json: [
        {
          organization_id: 1,
          organization_name: 'Atlas Field Consulting',
          role: 'admin',
          status: 'active',
        },
      ],
    });
  });

  await page.route(
    '**/api/admin/users/2/force-password-reset',
    async (route) => {
      await route.fulfill({ json: { success: true } });
    },
  );

  await page.route('**/api/admin/users/2/sessions', async (route) => {
    await route.fulfill({ json: { success: true } });
  });

  await page.goto('/platform/users/2');

  await expect(
    page.getByRole('heading', { name: /platform user detail/i }),
  ).toBeVisible();

  await expect(page.getByText('new.admin@example.com')).toBeVisible();
  await expect(page.getByText('Atlas Field Consulting')).toBeVisible();

  await page.getByRole('button', { name: /force password reset/i }).click();
  await expect(page.getByText(/password reset forced/i)).toBeVisible();

  await page.getByRole('button', { name: /revoke sessions/i }).click();
  await expect(page.getByText(/sessions revoked/i)).toBeVisible();
});

test('super admin can view organization detail and assign users there', async ({
  page,
}) => {
  let members: any[] = [];

  await seedSuperAdmin(page);

  await page.route('**/api/notifications', async (route) => {
    await route.fulfill({ json: [] });
  });

  await page.route('**/api/platform/users', async (route) => {
    await route.fulfill({
      json: [
        {
          id: 2,
          email: 'new.admin@example.com',
          name: 'New Admin',
          user_type: 'admin',
        },
      ],
    });
  });

  await page.route('**/api/platform/organizations/1/members', async (route) => {
    if (route.request().method() === 'POST') {
      members = [
        {
          id: 1,
          email: 'new.admin@example.com',
          name: 'New Admin',
          user_type: 'admin',
          role: 'finance_admin',
          status: 'active',
        },
      ];

      await route.fulfill({ json: members[0] });
      return;
    }

    await route.fulfill({ json: members });
  });

  await page.goto('/platform/organization/1');

  await expect(
    page.getByRole('heading', { name: /organization detail/i }),
  ).toBeVisible();

  await page.locator('select').first().selectOption('2');
  await page.locator('select').nth(1).selectOption('finance_admin');

  await page.getByRole('button', { name: /assign user/i }).click();

  await expect(page.getByText(/user assigned/i)).toBeVisible();
  const memberRow = page
    .locator('.table-row')
    .filter({ hasText: 'new.admin@example.com' });

  await expect(memberRow).toBeVisible();
  await expect(memberRow).toContainText('finance_admin');
  await expect(memberRow).toContainText('active');
});

test('normal admin cannot access platform admin controls', async ({ page }) => {
  await seedNormalAdmin(page);

  await page.route('**/api/notifications', async (route) => {
    await route.fulfill({ json: [] });
  });

  await page.goto('/platform');

  await expect(
    page
      .getByText(/platform admin access required/i)
      .or(page.getByRole('heading', { name: /you do not have permission/i })),
  ).toBeVisible();

  await expect(
    page.getByRole('button', { name: /create organization/i }),
  ).toHaveCount(0);
});
