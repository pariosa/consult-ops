import { useApi } from './useApi';

export interface PlatformOrganizationMember {
  id: number;
  organization_id: number;
  user_id: number;
  role: string;
  status: string;
  email?: string | null;
  name?: string | null;
  created_at?: string;
}
export interface PlatformOrganization {
  id: number;
  name: string;
  slug?: string | null;
}

export interface PlatformUser {
  id: number;
  email: string;
  name?: string | null;
  user_type: string;
}

export function usePlatformAdmin() {
  const api = useApi();

 
  async function createOrganization(payload: { name: string }) {
    return await api.post('/api/platform/organizations', payload);
  }

 
  async function getOrganizations(): Promise<PlatformOrganization[]> {
    return api.get<PlatformOrganization[]>('/api/platform/organizations');
  }

  async function getUsers(): Promise<PlatformUser[]> {
    return api.get<PlatformUser[]>('/api/platform/users');
  }

  async function createUser(payload: {
    email: string;
    name?: string;
    user_type: string;
    password: string;
  }) {
    return await api.post('/api/platform/users', payload);
  }

  async function getOrganizationMembers(
    organizationId: number,
  ): Promise<PlatformOrganizationMember[]> {
    return api.get<PlatformOrganizationMember[]>(
      `/api/platform/organizations/${organizationId}/members`,
    );
  }

  async function assignUserToOrganization(
    organizationId: number,
    payload: { user_id: number; role: string },
  ) {
    return await api.post(
      `/api/platform/organizations/${organizationId}/members`,
      payload,
    );
  }

  return {
    getOrganizations,
    createOrganization,
    getUsers,
    createUser,
    getOrganizationMembers,
    assignUserToOrganization,
  };
}
