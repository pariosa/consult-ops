import { useApi } from './useApi';

export function useOrganizationMembers() {
  const api = useApi();

  async function getMembers(organizationId: number) {
    return await api.get(`/api/organizations/${organizationId}/members`);
  }

  async function getInvitations(organizationId: number) {
    return await api.get(`/api/organizations/${organizationId}/invitations`);
  }

  async function inviteMember(
    organizationId: number,
    payload: { email: string; role: string },
  ) {
    return await api.post(
      `/api/organizations/${organizationId}/invitations`,
      payload,
    );
  }

  async function acceptInvitation(token: string) {
    return await api.post(
      `/api/organization-invitations/accept?token=${token}`,
      {},
    );
  }

  return {
    getMembers,
    getInvitations,
    inviteMember,
    acceptInvitation,
  };
}
