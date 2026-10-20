// frontend/composables/useAgreements.ts
import { useApi } from '~/composables/useApi';

export type CreateAgreementPayload = {
  engagement_id: number;
  title: string;
  agreement_type: string;
};

export type CreatePayoutRulePayload = {
  from_party_id: number;
  to_party_id: number;
  rule_type: string;
  percent?: number | null;
  amount_cents?: number | null;
  trigger_event: string;
};
export interface Agreement {
  id: number;
  organization_id: number;
  engagement_id: number;
  title: string;
  agreement_type: string;
  status: string;
  created_at?: string;
}

export interface PayoutRule {
  id: number;
  agreement_id: number;
  from_party_id: number;
  to_party_id: number;
  rule_type: string;
  percent?: number | null;
  amount_cents?: number | null;
  trigger_event: string;
}
export function useAgreements() {
  const api = useApi();

  return {
    listOrganizationAgreements: (
      organizationId: number,
    ): Promise<Agreement[]> =>
      api.get<Agreement[]>(
        `/api/organizations/${organizationId}/agreements`,
      ),

    createAgreement: (
      organizationId: number,
      payload: CreateAgreementPayload,
    ): Promise<Agreement> =>
      api.post<Agreement>(
        `/api/organizations/${organizationId}/agreements`,
        payload,
      ),

    lockAgreement: (agreementId: number): Promise<Agreement> =>
      api.post<Agreement>(
        `/api/agreements/${agreementId}/lock`,
        {},
      ),

    listPayoutRules: (agreementId: number): Promise<PayoutRule[]> =>
      api.get<PayoutRule[]>(
        `/api/agreements/${agreementId}/payout-rules`,
      ),

    createPayoutRule: (
      agreementId: number,
      payload: CreatePayoutRulePayload,
    ): Promise<PayoutRule> =>
      api.post<PayoutRule>(
        `/api/agreements/${agreementId}/payout-rules`,
        payload,
      ),
  };
}

