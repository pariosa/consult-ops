import { useApi } from './useApi';

export interface OrganizationClient {
  id: number;
  name: string;
  email: string | null;
  company?: string | null;
  phone?: string | null;
}

export interface CreateOrganizationClientPayload {
  name: string;
  email: string;
  company?: string;
  phone?: string;
}

export function useOrganizationClients() {
  const api = useApi();

  const getOrganizationClients = (
    organizationId: number,
  ): Promise<OrganizationClient[]> => {
    return api.get<OrganizationClient[]>(
      `/api/organizations/${organizationId}/clients`,
    );
  };

  const createOrganizationClient = (
    organizationId: number,
    payload: CreateOrganizationClientPayload,
  ) => {
    return api.post(
      `/api/organizations/${organizationId}/clients`,
      payload,
    );
  };

  const createVerifiedClientParty = (
    organizationId: number,
    clientId: number,
  ) => {
    return api.post(
      `/api/organizations/${organizationId}/parties/from-client/${clientId}`,
      {},
    );
  };

  return {
    getOrganizationClients,
    createOrganizationClient,
    createVerifiedClientParty,
  };
}