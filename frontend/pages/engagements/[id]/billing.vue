<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useEngagementBilling } from '~/composables/useEngagementBilling';

const route = useRoute();
const engagementId = computed(() => Number(route.params.id));

const { getEngagementBilling, createActivationCheckout } =
  useEngagementBilling();

const billingRecords = ref<any[]>([]);
const loading = ref(false);
const loadingCheckout = ref(false);
const error = ref('');

const activationBilling = computed(() =>
  billingRecords.value.find(
    (record) => record.billing_type === 'activation_fee',
  ),
);

const activationPaid = computed(
  () => activationBilling.value?.status === 'paid',
);

const checkoutStatus = computed(() => {
  if (route.query.checkout === 'success') {
    return 'Payment submitted. Waiting for Stripe confirmation.';
  }

  if (route.query.checkout === 'cancelled') {
    return 'Checkout was cancelled. No payment was recorded.';
  }

  return '';
});

async function loadBilling() {
  loading.value = true;
  error.value = '';

  try {
    billingRecords.value = await getEngagementBilling(engagementId.value);
  } catch (err: any) {
    error.value = err?.message || 'Failed to load engagement billing.';
  } finally {
    loading.value = false;
  }
}

async function startActivationCheckout() {
  loadingCheckout.value = true;
  error.value = '';

  try {
    const result = await createActivationCheckout(engagementId.value);
    const checkoutUrl = result?.url || result?.checkout_url;

    if (!checkoutUrl) {
      throw new Error('Stripe checkout URL was not returned.');
    }

    window.location.href = checkoutUrl;
  } catch (err: any) {
    error.value = err?.message || 'Unable to start Stripe checkout.';
  } finally {
    loadingCheckout.value = false;
  }
}

onMounted(loadBilling);
</script>

<template>
  <DashboardShell
    title="Engagement Billing"
    subtitle="Activation payments and Stripe checkout."
  >
    <main class="billing-page">
      <section class="billing-card">
        <div class="billing-header">
          <p class="billing-eyebrow">Billing</p>
          <h1>Activation Payment</h1>
          <p>
            Pay the platform activation fee through Stripe. Consult Ops only
            marks the fee paid after Stripe confirms the payment.
          </p>
        </div>

        <div v-if="checkoutStatus" class="notice">
          {{ checkoutStatus }}
        </div>

        <div v-if="error" class="error-box">
          {{ error }}
        </div>

        <div v-if="loading" class="status-panel">Loading billing status...</div>

        <div v-else class="payment-panel" :class="{ paid: activationPaid }">
          <div class="payment-copy">
            <p class="billing-eyebrow">Activation Fee</p>

            <h2 v-if="activationPaid">Activation fee paid</h2>
            <h2 v-else>Activation fee required</h2>

            <p class="status-line">
              Status:
              <span :class="activationPaid ? 'status-paid' : 'status-pending'">
                {{ activationBilling?.status || 'not started' }}
              </span>
            </p>

            <p v-if="activationPaid">
              Stripe has confirmed this payment. This engagement is financially
              cleared. Continue agreement setup to finish preparing the
              workflow.
            </p>

            <p v-else>
              You will be redirected to Stripe Checkout. The frontend return
              page does not mark this fee paid; the webhook confirmation does.
            </p>

            <p v-if="activationBilling" class="amount-line">
              Amount:
              <strong>
                ${{ (activationBilling.amount_cents / 100).toFixed(2) }}
                {{
                  activationBilling.currency?.toUpperCase?.() ||
                  activationBilling.currency
                }}
              </strong>
            </p>
          </div>

          <div class="payment-action">
            <NuxtLink
              v-if="activationPaid"
              :to="`/engagements/${engagementId}/agreements`"
              class="action-button"
            >
              Continue Agreement Setup
            </NuxtLink>

            <button
              v-else
              class="action-button"
              type="button"
              :disabled="loadingCheckout || !engagementId"
              @click="startActivationCheckout"
            >
              {{
                loadingCheckout
                  ? 'Redirecting to Stripe...'
                  : 'Pay Activation Fee'
              }}
            </button>
          </div>
        </div>

        <details
          v-if="activationBilling?.stripe_checkout_session_id"
          class="technical-details"
        >
          <summary>Technical payment details</summary>
          <p>
            Stripe session:
            {{ activationBilling.stripe_checkout_session_id }}
          </p>
        </details>
      </section>
    </main>
  </DashboardShell>
</template>

<style scoped>
.billing-page {
  max-width: 860px;
  margin: 0 auto;
  padding: 2rem;
}

.billing-card {
  border: 1px solid rgba(45, 212, 191, 0.24);
  border-radius: 20px;
  background: linear-gradient(
    180deg,
    rgba(15, 23, 42, 0.96),
    rgba(2, 12, 23, 0.98)
  );
  color: #e5eefc;
  padding: 28px;
  box-shadow: 0 22px 60px rgba(0, 0, 0, 0.32);
}

.billing-header {
  margin-bottom: 24px;
}

.billing-eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.13em;
  margin: 0 0 10px;
  text-transform: uppercase;
}

.billing-header h1,
.payment-copy h2 {
  color: #f8fafc;
  margin: 0 0 10px;
  letter-spacing: -0.03em;
}

.billing-header h1 {
  font-size: clamp(2rem, 4vw, 3rem);
}

.billing-header p,
.payment-copy p {
  color: #cbd5e1;
  line-height: 1.65;
  margin: 0;
}

.notice {
  border: 1px solid rgba(34, 211, 238, 0.28);
  border-radius: 14px;
  background: rgba(8, 47, 73, 0.38);
  color: #a5f3fc;
  margin-bottom: 18px;
  padding: 14px 16px;
}

.error-box {
  border: 1px solid rgba(248, 113, 113, 0.4);
  border-radius: 14px;
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
  margin-bottom: 18px;
  padding: 14px 16px;
}

.status-panel,
.payment-panel {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 18px;
  background: rgba(8, 31, 42, 0.88);
  padding: 20px;
}

.payment-panel {
  display: grid;
  gap: 20px;
}

.payment-panel.paid {
  border-color: rgba(52, 211, 153, 0.42);
  background: radial-gradient(
      circle at top right,
      rgba(52, 211, 153, 0.16),
      transparent 36%
    ),
    rgba(8, 31, 42, 0.9);
}

.status-line {
  margin: 12px 0 !important;
}

.status-paid,
.status-pending {
  font-weight: 900;
  text-transform: uppercase;
}

.status-paid {
  color: #6ee7b7;
}

.status-pending {
  color: #fbbf24;
}

.amount-line {
  margin-top: 14px !important;
}

.payment-action {
  display: flex;
  align-items: center;
}

.action-button {
  width: 100%;
  border: 0;
  border-radius: 14px;
  background: linear-gradient(90deg, #60a5fa, #34d399);
  color: #04111f;
  cursor: pointer;
  display: inline-flex;
  font-weight: 900;
  justify-content: center;
  padding: 14px 18px;
  text-decoration: none;
}

.action-button:disabled {
  cursor: not-allowed;
  filter: grayscale(0.4);
  opacity: 0.65;
}

.technical-details {
  border-top: 1px solid rgba(45, 212, 191, 0.16);
  color: #94a3b8;
  font-size: 0.8rem;
  margin-top: 18px;
  padding-top: 14px;
}

.technical-details p {
  overflow-wrap: anywhere;
}

@media (min-width: 760px) {
  .payment-panel {
    grid-template-columns: 1fr 260px;
    align-items: center;
  }
}
</style>
