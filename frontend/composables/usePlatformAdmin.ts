import { useApi } from './useApi';

export function usePlatformAdmin() {
  const api = useApi();

  async function getOrganizations() {
    return await api.get('/api/platform/organizations');
  }

  async function createOrganization(payload: { name: string }) {
    return await api.post('/api/platform/organizations', payload);
  }

  async function getUsers() {
    return await api.get('/api/platform/users');
  }

  async function createUser(payload: {
    email: string;
    name?: string;
    user_type: string;
    password: string;
  }) {
    return await api.post('/api/platform/users', payload);
  }

  async function getOrganizationMembers(organizationId: number) {
    return await api.get(
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
