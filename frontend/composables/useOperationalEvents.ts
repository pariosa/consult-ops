// frontend/composables/useOperationalEvents.ts

import { useApi } from './useApi';

export interface OperationalEvent {
  id: number;
  event_type: string;
  created_at: string;
  engagement_id?: number | null;
  organization_id?: number | null;
  metadata?: Record<string, unknown> | null;
}

export function useOperationalEvents() {
  const api = useApi();

  async function getEngagementEvents(engagementId: number) {
    return await api.get<OperationalEvent[]>(
      `/api/engagements/${engagementId}/events`,
    )
  }

  async function getOrganizationEvents(organizationId: number) {
    return await api.get<OperationalEvent[]>(
      `/api/engagements/${organizationId}/events`,
    )
  }

  return {
    getEngagementEvents,
    getOrganizationEvents,
  };
}
