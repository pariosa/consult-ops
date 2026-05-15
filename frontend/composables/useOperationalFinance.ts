import { useApi } from './useApi';

export function useOperationalFinance() {
  const api = useApi();

  async function getFinanceSummary(organizationId: number) {
    return await api.get(
      `/api/organizations/${organizationId}/finance-summary`,
    );
  }

  async function getPartyBalances(organizationId: number) {
    return await api.get(`/api/organizations/${organizationId}/party-balances`);
  }

  return {
    getFinanceSummary,
    getPartyBalances,
  };
}
