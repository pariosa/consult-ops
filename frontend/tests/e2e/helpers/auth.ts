// frontend/tests/e2e/helpers/auth.ts

import type { Page } from '@playwright/test';

export type MockAuthUser = {
  id: number;
  email: string;
  name: string;
  user_type: 'admin' | 'consultant' | 'client' | string;
  role: 'admin' | 'member' | 'owner' | string;
  portal: 'admin' | 'consultant' | 'client' | string;
  hasOrganization: boolean;
  organizationCount: number;
};

export type MockAuthOptions = Partial<MockAuthUser>;

export function buildMockAuthUser(options: MockAuthOptions = {}): MockAuthUser {
  return {
    id: options.id ?? 1,
    email: options.email ?? 'e2e@test.com',
    name: options.name ?? 'E2E User',
    user_type: options.user_type ?? 'consultant',
    role: options.role ?? 'admin',
    portal: options.portal ?? 'consultant',
    hasOrganization: options.hasOrganization ?? true,
    organizationCount: options.organizationCount ?? 1,
  };
}

export async function mockAuthUser(
  page: Page,
  options: MockAuthOptions = {},
): Promise<MockAuthUser> {
  const user = buildMockAuthUser(options);

  await page.addInitScript((authUser) => {
    const serialized = JSON.stringify(authUser);

    window.localStorage.setItem('auth:user', serialized);
    window.localStorage.setItem('user', serialized);
    window.localStorage.setItem('authUser', serialized);

    window.localStorage.setItem('token', 'e2e-token');
    window.localStorage.setItem('auth:token', 'e2e-token');
  }, user);

  return user;
}

export async function mockNoOrgUser(page: Page, options: MockAuthOptions = {}) {
  return mockAuthUser(page, {
    ...options,
    hasOrganization: false,
    organizationCount: 0,
  });
}

export async function mockSingleOrgUser(
  page: Page,
  options: MockAuthOptions = {},
) {
  return mockAuthUser(page, {
    ...options,
    hasOrganization: true,
    organizationCount: 1,
  });
}

export async function mockMultiOrgUser(
  page: Page,
  options: MockAuthOptions = {},
) {
  return mockAuthUser(page, {
    ...options,
    hasOrganization: true,
    organizationCount: 2,
  });
}

export async function mockAdminUser(page: Page, options: MockAuthOptions = {}) {
  return mockAuthUser(page, {
    ...options,
    user_type: 'admin',
    role: 'admin',
    portal: 'admin',
  });
}

export async function mockConsultantUser(
  page: Page,
  options: MockAuthOptions = {},
) {
  return mockAuthUser(page, {
    ...options,
    user_type: 'consultant',
    role: 'admin',
    portal: 'consultant',
  });
}

export async function mockClientUser(
  page: Page,
  options: MockAuthOptions = {},
) {
  return mockAuthUser(page, {
    ...options,
    user_type: 'client',
    role: 'member',
    portal: 'client',
  });
}
