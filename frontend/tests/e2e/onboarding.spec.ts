import { expect, test } from '@playwright/test';

const loginResponse = {
  token: 'e2e-token',
  user: {
    id: 101,
    email: 'new-user@example.test',
    name: 'New User',
    user_type: 'consultant',
  },
};

test.describe('Organization onboarding', () => {
  test('verified user with no org logs in and lands on onboarding', async ({
    page,
  }) => {
    await page.route('**/api/auth/login', async (route) => {
      await route.fulfill({ json: loginResponse });
    });

    await page.route('**/api/me/organizations', async (route) => {
      await route.fulfill({ json: [] });
    });

    await page.goto('/consultant-login');

    await page.getByLabel(/email/i).fill('new-user@example.test');
    await page.getByLabel(/^password$/i).fill('DemoPass123!');
    await page
      .getByRole('button', { name: /enter workspace|login|sign in/i })
      .click();

    await expect(page).toHaveURL(/\/onboarding/);
    await expect(
      page.getByRole('heading', { name: /create your organization/i }),
    ).toBeVisible();
  });

  test('onboarding creates org and redirects to project portal', async ({
    page,
  }) => {
    await page.addInitScript(() => {
      window.localStorage.setItem(
        'auth_user',
        JSON.stringify({
          id: 101,
          email: 'new-user@example.test',
          name: 'New User',
          user_type: 'consultant',
          role: 'consultant',
          token: 'e2e-token',
        }),
      );
    });

    await page.route('**/api/me/organizations', async (route) => {
      if (route.request().method() === 'POST') {
        await route.fulfill({
          status: 201,
          json: {
            organization_id: 99,
            name: 'Atlas Studio',
            slug: 'atlas-studio',
            role: 'owner',
            status: 'active',
            is_current: true,
          },
        });
        return;
      }

      await route.fulfill({ json: [] });
    });

    await page.goto('/onboarding');

    await page.getByLabel(/organization name/i).fill('Atlas Studio');
    await page.getByRole('button', { name: /create organization/i }).click();

    await expect(page).toHaveURL(/\/project-portal/);
  });

  test('user with multiple orgs logs in and lands on workspace select', async ({
    page,
  }) => {
    await page.route('**/api/auth/login', async (route) => {
      await route.fulfill({ json: loginResponse });
    });

    await page.route('**/api/me/organizations', async (route) => {
      await route.fulfill({
        json: [
          {
            organization_id: 1,
            name: 'Atlas Studio',
            role: 'owner',
            status: 'active',
            is_current: false,
          },
          {
            organization_id: 2,
            name: 'Verdant Systems',
            role: 'admin',
            status: 'active',
            is_current: false,
          },
        ],
      });
    });

    await page.goto('/consultant-login');

    await page.getByLabel(/email/i).fill('multi@example.test');
    await page.getByLabel(/^password$/i).fill('DemoPass123!');
    await page
      .getByRole('button', { name: /enter workspace|login|sign in/i })
      .click();

    await expect(page).toHaveURL(/\/workspace-select/);
    await expect(page.getByText('Atlas Studio')).toBeVisible();
    await expect(page.getByText('Verdant Systems')).toBeVisible();
  });

  test('workspace select chooses org and redirects to project portal', async ({
    page,
  }) => {
    await page.addInitScript(() => {
      window.localStorage.setItem(
        'auth_user',
        JSON.stringify({
          id: 101,
          email: 'multi@example.test',
          name: 'Multi User',
          user_type: 'consultant',
          role: 'consultant',
          token: 'e2e-token',
        }),
      );
    });

    await page.route('**/api/me/organizations', async (route) => {
      await route.fulfill({
        json: [
          {
            organization_id: 1,
            name: 'Atlas Studio',
            role: 'owner',
            status: 'active',
            is_current: false,
          },
          {
            organization_id: 2,
            name: 'Verdant Systems',
            role: 'admin',
            status: 'active',
            is_current: false,
          },
        ],
      });
    });

    await page.route('**/api/me/current-organization', async (route) => {
      await route.fulfill({
        json: {
          message: 'Current organization updated',
          organization_id: 2,
        },
      });
    });

    await page.goto('/workspace-select');

    await page.getByRole('button', { name: /verdant systems/i }).click();

    await expect(page).toHaveURL(/\/project-portal/);
  });
});
