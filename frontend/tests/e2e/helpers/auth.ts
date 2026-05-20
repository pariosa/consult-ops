// frontend/tests/e2e/helpers/auth.ts

import type { Page } from '@playwright/test';

export async function mockAuthUser(
  page: Page,
  user: {
    id: number;
    email: string;
    name?: string;
    user_type: string;
    role?: string;
    portal?: string;
  },
) {
  await page.addInitScript((authUser) => {
    const serialized = JSON.stringify(authUser);

    window.localStorage.setItem('auth:user', serialized);
    window.localStorage.setItem('user', serialized);
    window.localStorage.setItem('authUser', serialized);

    window.localStorage.setItem('token', 'e2e-token');
    window.localStorage.setItem('auth:token', 'e2e-token');
  }, user);
}
