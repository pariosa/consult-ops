import { useApi } from './useApi';

export interface Engagement {
  id: number;
  project_id: number;
  organization_id?: number | null;
  client_id?: number | null;
  title?: string | null;
  status?: string | null;
  engagement_type?: string | null;
  contractor_email?: string | null;
}

export interface TransactionReadiness {
  has_agreement: boolean;
  has_payout_rules: boolean;
  ready: boolean;
}

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
  const getTransactionReadiness = (
    engagementId: number,
  ): Promise<TransactionReadiness> =>
    get<TransactionReadiness>(
      `/api/engagements/${engagementId}/transaction-readiness`,
    );

  const getProjectEngagements = (
    projectId: number,
  ): Promise<Engagement[]> =>
    get<Engagement[]>(`/api/projects/${projectId}/engagements`);

  const getEngagement = (id: number): Promise<Engagement> =>
    get<Engagement>(`/api/engagements/${id}`);

  const createEngagement = (
    projectId: number,
    payload: any,
  ): Promise<Engagement> =>
    post<Engagement>(
      `/api/projects/${projectId}/engagements`,
      payload,
  );

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
