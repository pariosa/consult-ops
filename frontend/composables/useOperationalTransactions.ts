import { useApi } from './useApi';

export interface OperationalTransaction {
  id: number;
  organization_id?: number | null;
  engagement_id?: number | null;
  status: string;
  amount_cents?: number | null;
  created_at?: string;
}

export function useOperationalTransactions() {
  const api = useApi();

  async function getOrganizationTransactions(
    organizationId: number,
  ): Promise<OperationalTransaction[]> {
    return api.get<OperationalTransaction[]>(
      `/api/organizations/${organizationId}/transactions`,
    );
  }

  async function getEngagementTransactions(
    engagementId: number,
  ): Promise<OperationalTransaction[]> {
    return api.get<OperationalTransaction[]>(
      `/api/engagements/${engagementId}/transactions`,
    );
  }

  async function markProcessing(id: number) {
    return await api.post(
      `/api/operational-transactions/${id}/mark-processing`,
      {},
    );
  }

  async function markPaid(id: number) {
    return await api.post(`/api/operational-transactions/${id}/mark-paid`, {});
  }

  async function markFailed(id: number) {
    return await api.post(
      `/api/operational-transactions/${id}/mark-failed`,
      {},
    );
  }

  async function cancelTransaction(id: number) {
    return await api.post(`/api/operational-transactions/${id}/cancel`, {});
  }

  return {
    getOrganizationTransactions,
    getEngagementTransactions,
    markProcessing,
    markPaid,
    markFailed,
    cancelTransaction,
  };
}
