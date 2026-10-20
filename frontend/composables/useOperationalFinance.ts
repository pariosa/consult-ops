import { useApi } from './useApi';

export interface FinanceSummary {
  pending_cents: number;
  processing_cents: number;
  paid_cents: number;
  total_obligations_cents: number;
}

export interface PartyBalance {
  party_id: number;
  party_name: string;
  party_type: string;
  is_verified: boolean;
  payable_cents: number;
  receivable_cents: number;
  net_cents: number;
}

export function useOperationalFinance() {
  const api = useApi();

  async function getFinanceSummary(
    organizationId: number,
  ): Promise<FinanceSummary> {
    return api.get<FinanceSummary>(
      `/api/organizations/${organizationId}/finance-summary`,
    );
  }

  async function getPartyBalances(
    organizationId: number,
  ): Promise<PartyBalance[]> {
    return api.get<PartyBalance[]>(
      `/api/organizations/${organizationId}/party-balances`,
    );
  }

  return {
    getFinanceSummary,
    getPartyBalances,
  };
}