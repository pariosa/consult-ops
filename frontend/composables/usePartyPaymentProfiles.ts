import { useApi } from './useApi';

export function usePartyPaymentProfiles() {
  const api = useApi();

  return {
    getReadiness: (partyId: number) =>
      api.get(`/api/parties/${partyId}/payment-readiness`),

    upsertProfile: (
      partyId: number,
      payload: {
        payment_role: 'payer' | 'payee' | 'both';
        payer_authorization_scope?:
          | 'single_milestone'
          | 'engagement'
          | 'agreement';
      },
    ) => api.post(`/api/parties/${partyId}/payment-profile`, payload),

    verifyParty: (partyId: number) =>
      api.post(`/api/parties/${partyId}/verify`, {}),

    markPayoutReadyDev: (partyId: number) =>
      api.post(`/api/parties/${partyId}/payout-ready/dev`, {}),

    markPayerAuthorizedDev: (partyId: number) =>
      api.post(`/api/parties/${partyId}/payer-authorized/dev`, {}),
  };
}
