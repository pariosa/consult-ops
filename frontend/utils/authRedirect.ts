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
