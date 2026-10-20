import { useApi } from '../composables/useApi';

export interface Client {
  id: number;
  name: string;
  email?: string | null;
}

export function useClients() {
  const { get, post } = useApi();

  async function getOrganizationClients(
    organizationId: number,
  ): Promise<Client[]> {
    return get<Client[]>(`/api/organizations/${organizationId}/clients`);
  }

  async function createClient(
    organizationId: number,
    payload: any,
  ): Promise<Client> {
    return post<Client>(
      `/api/organizations/${organizationId}/clients`,
      payload,
    );
  }
  return {
    getOrganizationClients,
    createClient,
  };
}
