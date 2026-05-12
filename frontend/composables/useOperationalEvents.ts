// frontend/composables/useOperationalEvents.ts

import { useApi } from './useApi';

export function useOperationalEvents() {
  const api = useApi();

  async function getEngagementEvents(engagementId: number) {
    return await api.get(`/api/engagements/${engagementId}/events`);
  }

  async function getOrganizationEvents(organizationId: number) {
    return await api.get(`/api/organizations/${organizationId}/events`);
  }

  return {
    getEngagementEvents,
    getOrganizationEvents,
  };
}
