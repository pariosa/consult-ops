// frontend/composables/useEngagementBilling.ts
import { useApi } from './useApi';

export interface EngagementBillingItem {
  id: number;
  status: string;
  amount_cents?: number | null;
  billing_type?: string | null;
  stripe_checkout_session_id?: string | null;
}

export interface ActivationCheckoutResponse {
  url?: string;
  checkout_url?: string;
}

export function useEngagementBilling() {
  const api = useApi();

  async function getEngagementBilling(
    engagementId: number,
  ): Promise<EngagementBillingItem[]> {
    return api.get<EngagementBillingItem[]>(
      `/api/engagements/${engagementId}/billing`,
    );
  }

  async function createActivationCheckout(
    engagementId: number,
  ): Promise<ActivationCheckoutResponse> {
    return api.post<ActivationCheckoutResponse>(
      `/api/engagements/${engagementId}/activation-checkout`,
      {},
    );
  }

  async function createActivationFee(engagementId: number) {
    return await api.post(
      `/api/engagements/${engagementId}/activation-fee`,
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
