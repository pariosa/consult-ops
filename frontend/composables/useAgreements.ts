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

export function useAgreements() {
  const api = useApi();

  return {
    listOrganizationAgreements: (organizationId: number) =>
      api.get(`/api/organizations/${organizationId}/agreements`),

    createAgreement: (
      organizationId: number,
      payload: CreateAgreementPayload,
    ) => api.post(`/api/organizations/${organizationId}/agreements`, payload),

    lockAgreement: (agreementId: number) =>
      api.post(`/api/agreements/${agreementId}/lock`, {}),

    listPayoutRules: (agreementId: number) =>
      api.get(`/api/agreements/${agreementId}/payout-rules`),

    createPayoutRule: (agreementId: number, payload: CreatePayoutRulePayload) =>
      api.post(`/api/agreements/${agreementId}/payout-rules`, payload),
  };
}
