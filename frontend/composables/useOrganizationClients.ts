import { useApi } from './useApi';

export function useOrganizationClients() {
  const api = useApi();

  return {
    getOrganizationClients: (organizationId: number) =>
      api.get(`/api/organizations/${organizationId}/clients`),

    createOrganizationClient: (
      organizationId: number,
      payload: {
        name: string;
        email: string;
        company?: string;
        phone?: string;
      },
    ) => api.post(`/api/organizations/${organizationId}/clients`, payload),

    createVerifiedClientParty: (organizationId: number, clientId: number) =>
      api.post(
        `/api/organizations/${organizationId}/parties/from-client/${clientId}`,
        {},
      ),
  };
}
