import { useApi } from '~/composables/useApi';

export function useClients() {
  const { get, post } = useApi();

  async function getOrganizationClients(organizationId: number) {
    return await get(`/api/organizations/${organizationId}/clients`);
  }

  async function createClient(organizationId: number, payload: any) {
    return await post(`/api/organizations/${organizationId}/clients`, payload);
  }

  return {
    getOrganizationClients,
    createClient,
  };
}
