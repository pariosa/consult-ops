// frontend/utils/authRedirect.ts
// frontend/utils/authRedirect.ts
export type PortalType = 'admin' | 'consultant' | 'client';

export const getPortalRoute = (portal: string) => {
  const routes: Record<PortalType, string> = {
    admin: '/admin',
    consultant: '/project-portal',
    client: '/client-portal',
  };

  return routes[portal as PortalType] ?? '/unauthorized';
};

export function resolvePostLoginRoute(user: any) {
  if (!user?.hasOrganization || user.organizationCount === 0) {
    return '/onboarding';
  }

  if (user.organizationCount > 1) {
    return '/workspace-select';
  }

  return '/project-portal';
}
