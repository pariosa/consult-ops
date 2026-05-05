import { useApi } from './useApi';

export function useEngagements() {
  const api = useApi();

  const getProjectEngagements = (projectId: number) =>
    api.get(`/api/projects/${projectId}/engagements`);

  const getEngagement = (id: number) => api.get(`/api/engagements/${id}`);

  const createEngagement = (projectId: number, payload: any) =>
    api.post(`/api/projects/${projectId}/engagements`, payload);

  const generateSoftwareContract = (id: number) =>
    api.post(`/api/engagements/${id}/software-contract`, {});

  const markContractSent = (id: number) =>
    api.post(`/api/engagements/${id}/mark-contract-sent`, {});

  const markSigned = (id: number) =>
    api.post(`/api/engagements/${id}/mark-signed`, {});

  return {
    getProjectEngagements,
    getEngagement,
    createEngagement,
    generateSoftwareContract,
    markContractSent,
    markSigned,
  };
}
