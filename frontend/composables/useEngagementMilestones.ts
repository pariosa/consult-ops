import { useApi } from './useApi';

export interface EngagementMilestone {
  id: number;
  engagement_id: number;
  title: string;
  status: string;
}

export function useEngagementMilestones() {
  const api = useApi();

  const getMilestones = (engagementId: number) =>
    api.get<EngagementMilestone[]>(
      `/api/engagements/${engagementId}/milestones`,
    );

  const createMilestone = (engagementId: number, payload: any) =>
    api.post(`/api/engagements/${engagementId}/milestones`, payload);

  const submitMilestone = (id: number) =>
    api.post(`/api/milestones/${id}/submit`, {});

  const approveMilestone = (id: number) =>
    api.post(`/api/milestones/${id}/approve`, {});

  const markMilestonePaid = (id: number) =>
    api.post(`/api/milestones/${id}/mark-paid`, {});

  async function updateMilestone(id: number, payload: any) {
    await api.patch(`/api/engagements/milestones/${id}`, payload);
  }

  async function reopenMilestone(id: number) {
    return api.get(`/api/engagements/milestones/${id}/reopen`, {
      method: 'POST',
    });
  }
  return {
    getMilestones,
    createMilestone,
    submitMilestone,
    approveMilestone,
    markMilestonePaid,
    updateMilestone,
    reopenMilestone,
  };
}
