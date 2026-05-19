import { useApi } from './useApi';

export function useOperationalTransactions() {
  const api = useApi();

  async function getOrganizationTransactions(organizationId: number) {
    return await api.get(`/api/organizations/${organizationId}/transactions`);
  }

  async function getEngagementTransactions(engagementId: number) {
    return await api.get(`/api/engagements/${engagementId}/transactions`);
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
