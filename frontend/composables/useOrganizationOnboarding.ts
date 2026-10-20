
import { useApi } from './useApi';

export type MyOrganization = {
  organization_id: number;
  name: string;
  slug?: string | null;
  role: string;
  status: string;
  is_current: boolean;
};

export type DiscoverableOrganization = {
  id: number;
  name: string;
  slug?: string | null;
};

export type MyJoinRequest = {
  id: number;
  organization_id: number;
  organization_name: string;
  status: 'pending' | 'approved' | 'rejected' | 'cancelled';
  created_at: string;
};

export type OrganizationJoinRequest = {
  id: number;
  organization_id: number;
  user_id: number;
  requested_role: string;
  status: string;
  created_at: string;
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

  const discoverOrganizations = () =>
    api.get<DiscoverableOrganization[]>('/api/organizations/discover');

  const getMyJoinRequests = () =>
    api.get<MyJoinRequest[]>('/api/me/join-requests');

  const requestToJoin = (organizationId: number) =>
    api.post<OrganizationJoinRequest>(
      `/api/organizations/${organizationId}/join-requests`,
      {},
    );

  const getOrganizationJoinRequests = (organizationId: number) =>
    api.get<OrganizationJoinRequest[]>(
      `/api/organizations/${organizationId}/join-requests`,
    );

  const approveJoinRequest = (
    organizationId: number,
    requestId: number,
  ) =>
    api.post(
      `/api/organizations/${organizationId}/join-requests/${requestId}/approve`,
      {},
    );

  const rejectJoinRequest = (
    organizationId: number,
    requestId: number,
  ) =>
    api.post(
      `/api/organizations/${organizationId}/join-requests/${requestId}/reject`,
      {},
    );

  return {
    getMyOrganizations,
    createOrganization,
    setCurrentOrganization,
    discoverOrganizations,
    getMyJoinRequests,
    requestToJoin,
    getOrganizationJoinRequests,
    approveJoinRequest,
    rejectJoinRequest,
  };
};
