// frontend/tests/e2e/helpers/auth.ts

import type { Page } from '@playwright/test';

export type MockAuthOptions = {
  id?: number;
  email?: string;
  name?: string;
  user_type?: string;
  role?: string;
  portal?: string;
  organizationCount?: number;
  hasOrganization?: boolean;
};

export async function mockAuthUser(page: Page, options: MockAuthOptions = {}) {
  const user = {
    id: options.id ?? 1,
    email: options.email ?? 'e2e@test.com',
    name: options.name ?? 'E2E User',
    user_type: options.user_type ?? 'consultant',
    role: options.role ?? 'admin',
    portal: options.portal ?? 'consultant',
    organizationCount: options.organizationCount ?? 1,
    hasOrganization: options.hasOrganization ?? true,
  };

  await page.addInitScript((authUser) => {
    const serialized = JSON.stringify(authUser);

    window.localStorage.setItem('auth:user', serialized);
    window.localStorage.setItem('user', serialized);
    window.localStorage.setItem('authUser', serialized);

    window.localStorage.setItem('token', 'e2e-token');
    window.localStorage.setItem('auth:token', 'e2e-token');
  }, user);
}

export async function mockNoOrgUser(page: Page) {
  await mockAuthUser(page, {
    organizationCount: 0,
    hasOrganization: false,
  });
}

export async function mockSingleOrgUser(page: Page) {
  await mockAuthUser(page, {
    organizationCount: 1,
    hasOrganization: true,
  });
}

export async function mockMultiOrgUser(page: Page) {
  await mockAuthUser(page, {
    organizationCount: 2,
    hasOrganization: true,
  });
}
