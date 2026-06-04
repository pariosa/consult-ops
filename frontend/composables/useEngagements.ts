import { useApi } from './useApi';

export function useEngagements() {
  const { get, post, apiFetch } = useApi();
  async function updateContractRecipient(id: number, contractor_email: string) {
    return await apiFetch(`/api/engagements/${id}/contract-recipient`, {
      method: 'PATCH',
      body: {
        contractor_email,
      },
    });
  }

  async function resendContract(id: number) {
    return await apiFetch(`/api/engagements/${id}/resend-contract`, {
      method: 'POST',
    });
  }
  const getTransactionReadiness = (engagementId: number) => {
    get(`/api/engagements/${engagementId}/transaction-readiness`);
  };
  const getProjectEngagements = (projectId: number) =>
    get(`/api/projects/${projectId}/engagements`);

  const getEngagement = (id: number) => get(`/api/engagements/${id}`);

  const createEngagement = (projectId: number, payload: any) =>
    post(`/api/projects/${projectId}/engagements`, payload);

  const generateSoftwareContract = (id: number) =>
    post(`/api/engagements/${id}/software-contract`, {});

  const markContractSent = (id: number) =>
    post(`/api/engagements/${id}/mark-contract-sent`, {});

  const markSigned = (id: number) =>
    post(`/api/engagements/${id}/mark-signed`, {});

  const completeEngagement = (id: number) =>
    post(`/api/engagements/${id}/complete`, {});

  return {
    getTransactionReadiness,
    updateContractRecipient,
    resendContract,
    completeEngagement,
    getProjectEngagements,
    getEngagement,
    createEngagement,
    generateSoftwareContract,
    markContractSent,
    markSigned,
  };
}
