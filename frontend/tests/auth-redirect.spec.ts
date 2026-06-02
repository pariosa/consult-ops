// frontend/tests/authRedirect.test.ts

import { describe, expect, it } from 'vitest';
import { getPortalRoute, resolvePostLoginRoute } from '../utils/authRedirect';

describe('authRedirect', () => {
  it('routes users with no organization to onboarding', () => {
    expect(
      resolvePostLoginRoute({
        hasOrganization: false,
        organizationCount: 0,
      }),
    ).toBe('/onboarding');
  });

  it('routes users with multiple organizations to workspace select', () => {
    expect(
      resolvePostLoginRoute({
        hasOrganization: true,
        organizationCount: 2,
      }),
    ).toBe('/workspace-select');
  });

  it('routes users with one organization to project portal', () => {
    expect(
      resolvePostLoginRoute({
        hasOrganization: true,
        organizationCount: 1,
      }),
    ).toBe('/project-portal');
  });

  it('routes missing user to onboarding', () => {
    expect(resolvePostLoginRoute(null)).toBe('/onboarding');
  });

  it('routes portal users to their portal home', () => {
    expect(getPortalRoute('admin')).toBe('/admin');
    expect(getPortalRoute('consultant')).toBe('/project-portal');
    expect(getPortalRoute('client')).toBe('/client-portal');
  });

  it('routes unknown portal users to unauthorized', () => {
    expect(getPortalRoute('wat')).toBe('/unauthorized');
  });
});
