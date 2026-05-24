import { useApi } from './useApi';

export type MyOrganization = {
  organization_id: number;
  name: string;
  slug?: string | null;
  role: string;
  status: string;
  is_current: boolean;
};

export const useOrganizationOnboarding = () => {
  const api = useApi();

  const getMyOrganizations = () =>
    api.get<MyOrganization[]>('/api/me/organizations');

  const createOrganization = (payload: { name: string }) =>
    api.post<MyOrganization>('/api/me/organizations', payload);

  const setCurrentOrganization = (organization_id: number) =>
    api.post<{ message: string; organization_id: number }>(
      '/api/me/current-organization',
      { organization_id },
    );

  return {
    getMyOrganizations,
    createOrganization,
    setCurrentOrganization,
  };
};
