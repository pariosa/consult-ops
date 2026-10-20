import { useApi } from './useApi';

export interface OrganizationMemberRecord {
  id: number;
  user_id: number;
  organization_id: number;
  role: string;
  email?: string;
  name?: string;
}

export interface OrganizationInvitation {
  id: number;
  email: string;
  role: string;
  status?: string;
  created_at?: string;
  expires_at?: string | null;
}

export function useOrganizationMembers() {
  const api = useApi();

  async function getMembers(
    organizationId: number,
  ): Promise<OrganizationMemberRecord[]> {
    return api.get<OrganizationMemberRecord[]>(
      `/api/organizations/${organizationId}/members`,
    );
  }

  async function getInvitations(
    organizationId: number,
  ): Promise<OrganizationInvitation[]> {
    return api.get<OrganizationInvitation[]>(
      `/api/organizations/${organizationId}/invitations`,
    );
  }

  async function inviteMember(
    organizationId: number,
    payload: { email: string; role: string },
  ) {
    return api.post(
      `/api/organizations/${organizationId}/invitations`,
      payload,
    );
  }

  async function acceptInvitation(token: string) {
    return api.post(
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