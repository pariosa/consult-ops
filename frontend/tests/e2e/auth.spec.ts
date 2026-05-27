import { expect, test } from '@playwright/test';

const API_URL = process.env.API_URL ?? 'http://127.0.0.1:8000';

const seededConsultant = {
  email: 'contractor@atlas.test',
  password: 'DemoPass123!',
};

const uniqueEmail = () =>
  `auth-${Date.now()}-${Math.random().toString(36).slice(2)}@example.test`;

test.describe('Auth flow', () => {
  test('registers a user and requires email verification before login', async ({
    page,
  }) => {
    const email = uniqueEmail();

    await page.goto('/register');

    await page.getByLabel(/name/i).fill('Test User');
    await page.getByLabel(/email/i).fill(email);
    await page.getByLabel(/^Password$/i).fill('Password123!');
    await page.getByLabel(/^Confirm password$/i).fill('Password123!');

    await page
      .getByRole('button', { name: /register|create account|sign up/i })
      .click();

    await expect(
      page.getByText(
        /verify|verification|check your email|account created|registration/i,
      ),
    ).toBeVisible();

    await page.goto('/consultant-login');

    await page.getByLabel(/email/i).fill(email);
    await page.getByLabel(/^Password$/i).fill('Password123!');
    await page
      .getByRole('button', { name: /login|sign in|enter workspace/i })
      .click();

    await expect(
      page.getByText('Please verify your email before logging in.'),
    ).toBeVisible();
    await expect(
      page.getByRole('button', { name: /resend verification email/i }),
    ).toBeVisible();
  });

  test('logs in a verified seeded user and loads profile', async ({ page }) => {
    await page.goto('/consultant-login');

    await page.getByLabel(/email/i).fill(seededConsultant.email);
    await page.getByLabel(/^Password$/i).fill(seededConsultant.password);
    await page
      .getByRole('button', { name: /login|sign in|enter workspace/i })
      .click();

    await expect(page).not.toHaveURL(/consultant-login/);

    await page.goto('/profile');

    await expect(page.getByText(seededConsultant.email)).toBeVisible();
  });

  test('forgot password does not reveal whether an email exists', async ({
    page,
  }) => {
    await page.goto('/forgot-password');

    await page.getByLabel(/email/i).fill('missing-user@example.test');
    await page.getByRole('button', { name: /reset|send/i }).click();

    await expect(
      page.getByText(/if an account exists|password reset link has been sent/i),
    ).toBeVisible();
  });

  test('remember me creates a durable login session for seeded user', async ({
    page,
  }) => {
    await page.goto('/consultant-login');

    await page.getByLabel(/email/i).fill(seededConsultant.email);
    await page.getByLabel(/^Password$/i).fill(seededConsultant.password);

    const remember = page.getByLabel(/remember/i);
    await expect(remember).toBeVisible();
    await remember.check();

    await page
      .getByRole('button', { name: /login|sign in|enter workspace/i })
      .click();

    await expect
      .poll(async () =>
        page.evaluate(
          () =>
            window.localStorage.getItem('auth_user') ||
            window.localStorage.getItem('auth:user'),
        ),
      )
      .toContain(seededConsultant.email);

    await page.reload();

    await expect
      .poll(async () =>
        page.evaluate(
          () =>
            window.localStorage.getItem('auth_user') ||
            window.localStorage.getItem('auth:user'),
        ),
      )
      .toContain(seededConsultant.email);
  });

  test('user can view and revoke active sessions', async ({ page }) => {
    await page.goto('/consultant-login');

    await page.getByLabel(/email/i).fill(seededConsultant.email);
    await page.getByLabel(/^Password$/i).fill(seededConsultant.password);
    await page
      .getByRole('button', { name: /login|sign in|enter workspace/i })
      .click();

    await expect(page).not.toHaveURL(/consultant-login/);

    await page.goto('/profile');

    await expect(page.getByText(/active sessions|sessions/i)).toBeVisible();

    await page
      .getByRole('button', { name: /revoke|sign out session/i })
      .first()
      .click();

    await expect(page.getByText(/session revoked|signed out/i)).toBeVisible();
  });
});
