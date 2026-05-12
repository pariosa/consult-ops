// frontend/composables/useEngagementBilling.ts
import { useApi } from './useApi';

export function useEngagementBilling() {
  const api = useApi();

  async function getEngagementBilling(engagementId: number) {
    return await api.get(`/api/engagements/${engagementId}/billing`);
  }

  async function createActivationFee(engagementId: number) {
    return await api.post(
      `/api/engagements/${engagementId}/activation-fee`,
      {},
    );
  }

  async function createActivationCheckout(engagementId: number) {
    return await api.post(
      `/api/engagements/${engagementId}/activation-checkout`,
      {},
    );
  }

  async function attachCheckoutSession(
    billingId: number,
    stripeCheckoutSessionId: string,
  ) {
    return await api.patch(
      `/api/engagement-billing/${billingId}/checkout-session`,
      {
        stripe_checkout_session_id: stripeCheckoutSessionId,
      },
    );
  }

  async function markBillingPaid(billingId: number) {
    return await api.post(`/api/engagement-billing/${billingId}/mark-paid`, {});
  }
  return {
    getEngagementBilling,
    createActivationFee,
    createActivationCheckout,
    markBillingPaid,
    attachCheckoutSession,
  };
}
