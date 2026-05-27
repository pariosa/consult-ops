<!-- frontend/pages/engagements/[id]/index.vue-->
<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useEngagementMilestones } from '~/composables/useEngagementMilestones';
import { useEngagements } from '~/composables/useEngagements';
import EngagementTracker from '~/components/Engagements/EngagementTracker.vue';
import SoftwareContractPreview from '~/components/Contracts/SoftwareContractPreview.vue';
import OperationalTimeline from '~/components/Operations/OperationalTimeline.vue';
import { useOperationalEvents } from '~/composables/useOperationalEvents';
import EngagementNextActionCard from '~/components/Operations/EngagementNextActionCard.vue';
import { useEngagementBilling } from '~/composables/useEngagementBilling';

const { getEngagementEvents } = useOperationalEvents();
const route = useRoute();
const router = useRouter();
const engagementId = Number(route.params.id);

const engagement = ref<any>(null);
const milestones = ref<any[]>([]);
const contractPreview = ref('');
const loading = ref(true);
const success = ref('');

const recipientEmail = ref('');
const recipientSaving = ref(false);
const resendLoading = ref(false);
const checkoutLoading = ref(false);
const billingItems = ref<any[]>([]);
const operationalEvents = ref<any[]>([]);
const error = ref('');
const safeMilestones = computed(() =>
  Array.isArray(milestones.value) ? milestones.value : [],
);
const { getEngagementBilling, createActivationCheckout, markBillingPaid } =
  useEngagementBilling();

const {
  getEngagement,
  generateSoftwareContract,
  markContractSent,
  markSigned,
  completeEngagement,
  updateContractRecipient,
  resendContract,
} = useEngagements();

const { getMilestones, submitMilestone, approveMilestone, markMilestonePaid } =
  useEngagementMilestones();

async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    engagement.value = await getEngagement(engagementId);
    recipientEmail.value = engagement.value?.contractor_email || '';
    milestones.value = await getMilestones(engagementId);
    operationalEvents.value = await getEngagementEvents(engagementId);
    billingItems.value = await getEngagementBilling(engagementId);
  } catch (err: any) {
    error.value = err?.message || 'Failed to load engagement.';
  } finally {
    loading.value = false;
  }
}

async function previewContract() {
  const res: any = await generateSoftwareContract(engagementId);
  contractPreview.value = res.body;
}
function goToAgreements() {
  router.push(`/engagements/${engagementId}/agreements`);
}

async function submitMilestoneLocal(id: number) {
  await submitMilestone(id);
  await refresh();
}

async function approveMilestoneLocal(id: number) {
  await approveMilestone(id);
  await refresh();
}

async function markPaidLocal(id: number) {
  await markMilestonePaid(id);
  await refresh();
}
function goToTransactions() {
  router.push(`/engagements/${engagementId}/transactions`);
}
function goToMilestones() {
  router.push(`/engagements/${engagementId}/milestones`);
}

function goToBilling() {
  router.push(`/engagements/${engagementId}/billing`);
}
async function createActivationFeeLocal() {
  await createActivationCheckout(engagementId);
  await refresh();
}

async function markBillingPaidLocal(billingId: number) {
  await markBillingPaid(billingId);
  await refresh();
}
async function payActivationFee() {
  checkoutLoading.value = true;
  error.value = '';

  try {
    const res: any = await createActivationCheckout(engagementId);

    if (res?.url) {
      window.location.href = res.url;
      return;
    }

    error.value =
      'Stripe checkout session was created, but no checkout URL was returned.';
  } catch (err: any) {
    error.value =
      err?.data?.error ||
      err?.message ||
      'Failed to start activation checkout.';
  } finally {
    checkoutLoading.value = false;
  }
}
async function updateRecipientLocal() {
  recipientSaving.value = true;
  error.value = '';
  success.value = '';

  try {
    await updateContractRecipient(engagementId, recipientEmail.value);
    success.value = 'Contract recipient updated.';
    await refresh();
  } catch (err: any) {
    error.value =
      err?.data?.error ||
      err?.message ||
      'Failed to update contract recipient.';
  } finally {
    recipientSaving.value = false;
  }
}

async function resendContractLocal() {
  resendLoading.value = true;
  error.value = '';
  success.value = '';

  try {
    await resendContract(engagementId);
    success.value = 'Contract resent.';
    await refresh();
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to resend contract.';
  } finally {
    resendLoading.value = false;
  }
}

async function sendContract() {
  error.value = '';
  success.value = '';

  try {
    await markContractSent(engagementId);
    success.value = 'Contract marked sent.';
    await refresh();
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to mark contract sent.';
  }
}

async function markSignedLocal() {
  error.value = '';
  success.value = '';

  try {
    await markSigned(engagementId);
    success.value = 'Contract marked signed.';
    await refresh();
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to mark contract signed.';
  }
}

async function completeEngagementLocal() {
  error.value = '';
  success.value = '';

  try {
    await completeEngagement(engagementId);
    success.value = 'Engagement completed.';
    await refresh();
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to complete engagement.';
  }
}

const canSendOrResendContract = computed(() => {
  return ![
    'contract_signed',
    'active',
    'paid',
    'completed',
    'cancelled',
  ].includes(engagement.value?.status);
});

const canEditContractDelivery = computed(() => {
  return ![
    'contract_signed',
    'active',
    'paid',
    'completed',
    'cancelled',
  ].includes(engagement.value?.status);
});

const canSendContract = computed(() => {
  return ['draft', 'created'].includes(engagement.value?.status);
});

const canResendContract = computed(() => {
  return engagement.value?.status === 'contract_sent';
});

const canMarkContractSigned = computed(() => {
  return engagement.value?.status === 'contract_sent';
});

const canCompleteEngagement = computed(() => {
  return (
    engagement.value &&
    safeMilestones.value.length > 0 &&
    safeMilestones.value.every((m) =>
      ['paid', 'completed'].includes(m.status),
    ) &&
    engagement.value.status !== 'completed'
  );
});
onMounted(refresh);
</script>

<template>
  <DashboardShell
    title="Engagement Tracker"
    subtitle="Progress contract, milestone, and payment workflow."
  >
    <section v-if="loading" class="portal-section">
      Loading engagement...
    </section>

    <section v-else-if="error" class="form-error">
      {{ error }}
    </section>
    <template v-else-if="engagement">
      <section class="portal-section engagement-hero">
        <p class="eyebrow">{{ engagement.title }}</p>
        <h2>{{ engagement.deliverables }}</h2>
        <p class="contractor-line">
          {{ engagement.contractor_name }} — {{ engagement.contractor_email }}
        </p>
        <p class="scope-line">{{ engagement.scope_of_work }}</p>
      </section>
      <EngagementNextActionCard v-if="engagement" :engagement="engagement" />
      <EngagementTracker
        :status="engagement.status"
        :platform-fee-status="engagement.platform_fee_status"
        :milestones="safeMilestones"
      />
      <OperationalTimeline :events="operationalEvents" />

      <section class="portal-section">
        <div class="section-header">
          <div>
            <p class="eyebrow">Contract Delivery</p>
            <h2>Recipient & Resend Controls</h2>
            <p>
              Update the recipient or resend the contract before it is signed or
              activated.
            </p>
          </div>
        </div>

        <div class="delivery-grid">
          <div>
            <label for="contract-recipient-email">Recipient Email</label>
            <input
              id="contract-recipient-email"
              v-model="recipientEmail"
              class="form-input"
              type="email"
              :disabled="!canEditContractDelivery || recipientSaving"
            />
          </div>

          <button
            class="form-button"
            :disabled="
              !canEditContractDelivery || recipientSaving || !recipientEmail
            "
            @click="updateRecipientLocal"
          >
            {{ recipientSaving ? 'Saving...' : 'Update Recipient' }}
          </button>

          <button
            v-if="canSendContract"
            class="form-button"
            @click="sendContract"
          >
            Send Contract
          </button>

          <button
            v-if="canResendContract"
            class="form-button"
            :disabled="resendLoading"
            @click="resendContractLocal"
          >
            {{ resendLoading ? 'Resending...' : 'Resend Contract' }}
          </button>

          <button
            v-if="canMarkContractSigned"
            class="form-button"
            @click="markSignedLocal"
          >
            Mark Contract Signed
          </button>

          <button
            v-if="!canEditContractDelivery"
            class="form-button secondary"
            disabled
          >
            Contract delivery locked after signing or activation
          </button>
        </div>
      </section>
      <section class="portal-section action-grid">
        <button class="form-button" @click="previewContract">
          Generate Contract Preview
        </button>
        <button type="button" class="form-button" @click="goToAgreements">
          Agreement Rules
        </button>

        <button
          v-if="canMarkContractSigned"
          class="form-button"
          @click="markSignedLocal"
        >
          Mark Contract Signed
        </button>

        <button type="button" class="form-button" @click="goToMilestones">
          Manage Milestones
        </button>

        <button type="button" class="form-button" @click="goToBilling">
          Payment Workflow
        </button>
        <button type="button" class="form-button" @click="goToTransactions">
          Transaction Ledger
        </button>
        <button
          class="form-button"
          @click="router.push('/organization/finance')"
        >
          Finance Dashboard
        </button>
      </section>
      <section
        v-if="canCompleteEngagement"
        class="portal-section completion-panel"
      >
        <p class="eyebrow">Ready For Closeout</p>

        <h2>All Milestones Completed</h2>

        <p>
          All milestone obligations have been fulfilled and paid. Finalize the
          engagement to close the operational workflow.
        </p>

        <button
          class="form-button final-signoff"
          @click="completeEngagementLocal"
        >
          Final Sign-Off / Complete Engagement
        </button>
      </section>
      <SoftwareContractPreview v-if="contractPreview" :body="contractPreview" />

      <section class="portal-section">
        <div class="section-header">
          <h2>Milestones</h2>
          <p>Submit, approve, and mark work as paid.</p>
        </div>

        <div v-if="!safeMilestones.length">No milestones yet.</div>

        <div
          v-for="milestone in safeMilestones"
          :key="milestone.id"
          class="ops-card milestone-row"
        >
          <div>
            <h3>{{ milestone.title }}</h3>
            <p>{{ milestone.description }}</p>
            <p>Status: {{ milestone.status }}</p>
          </div>

          <div class="milestone-actions">
            <button @click="submitMilestoneLocal(milestone.id)">Submit</button>
            <button @click="approveMilestoneLocal(milestone.id)">
              Approve
            </button>
            <button @click="markPaidLocal(milestone.id)">Mark Paid</button>
          </div>
        </div>
      </section>
      <section class="portal-section">
        <div class="section-header">
          <div>
            <p class="eyebrow">Activation Billing</p>
            <h2>Engagement Access</h2>
            <p>
              Pay the activation fee to unlock the engagement workflow and
              record the payment in the operational timeline.
            </p>
          </div>

          <button
            class="form-button"
            :disabled="checkoutLoading || engagement.status === 'active'"
            @click="payActivationFee"
          >
            {{
              checkoutLoading ? 'Starting Checkout...' : 'Pay Activation Fee'
            }}
          </button>
        </div>

        <div v-if="!billingItems.length" class="empty-state">
          No activation billing record yet.
        </div>

        <div
          v-for="billing in billingItems"
          :key="billing.id"
          class="ops-card billing-row"
        >
          <div>
            <h3>{{ billing.billing_type }}</h3>
            <p>Status: {{ billing.status }}</p>
            <p>
              Amount: ${{ (billing.amount_cents / 100).toFixed(2) }}
              {{ billing.currency?.toUpperCase?.() || billing.currency }}
            </p>
            <p v-if="billing.stripe_checkout_session_id">
              Stripe session: {{ billing.stripe_checkout_session_id }}
            </p>
          </div>

          <button
            v-if="billing.status !== 'paid'"
            class="form-button secondary"
            @click="markBillingPaidLocal(billing.id)"
          >
            Dev: Mark Paid
          </button>
        </div>
      </section>
    </template>
  </DashboardShell>
</template>

<style scoped>
.portal-section {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 18px;
  background: linear-gradient(
    180deg,
    rgba(15, 23, 42, 0.96),
    rgba(2, 12, 23, 0.96)
  );
  box-shadow: 0 18px 50px rgba(0, 0, 0, 0.28);
  color: #e5eefc;
  padding: 24px;
  margin-bottom: 24px;
}

.portal-section h2,
.portal-section h3 {
  color: #f8fafc;
  margin: 0 0 10px;
  letter-spacing: -0.02em;
}

.portal-section p,
.portal-section div {
  color: #cbd5e1;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 800;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  margin-bottom: 12px;
}

.action-grid {
  display: grid;
  gap: 12px;
}

.form-button,
.milestone-actions button {
  width: 100%;
  border: 0;
  border-radius: 12px;
  background: linear-gradient(90deg, #60a5fa, #34d399);
  color: #04111f;
  cursor: pointer;
  font-weight: 800;
  padding: 14px 18px;
  transition:
    transform 0.14s ease,
    box-shadow 0.14s ease,
    filter 0.14s ease;
}

.form-button:hover,
.milestone-actions button:hover {
  box-shadow: 0 14px 30px rgba(52, 211, 153, 0.18);
  filter: brightness(1.06);
  transform: translateY(-1px);
}

.section-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 18px;
}

.section-header h2 {
  margin-bottom: 6px;
}

.ops-card {
  border: 1px solid rgba(45, 212, 191, 0.2);
  border-radius: 16px;
  background: rgba(8, 31, 42, 0.86);
  padding: 18px;
}

.milestone-row {
  display: grid;
  gap: 18px;
  margin-top: 14px;
}

.milestone-row h3 {
  font-size: 1.05rem;
}

.milestone-row p {
  margin: 7px 0;
  line-height: 1.55;
}

.milestone-actions {
  display: grid;
  gap: 10px;
}

.form-error {
  border: 1px solid rgba(248, 113, 113, 0.38);
  border-radius: 14px;
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
  padding: 16px;
  margin-bottom: 20px;
}

@media (min-width: 760px) {
  .action-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .milestone-row {
    grid-template-columns: 1fr 240px;
    align-items: center;
  }
}
.billing-row {
  display: grid;
  gap: 16px;
  margin-top: 14px;
}

.billing-row h3 {
  color: #f8fafc;
  margin: 0 0 8px;
}

.billing-row p {
  color: #cbd5e1;
  margin: 4px 0;
}

.form-button:disabled {
  cursor: not-allowed;
  filter: grayscale(0.4);
  opacity: 0.6;
}

@media (min-width: 760px) {
  .billing-row {
    grid-template-columns: 1fr 220px;
    align-items: center;
  }
}
.final-signoff {
  background: linear-gradient(90deg, #34d399, #a7f3d0);
}
.completion-panel {
  border: 1px solid rgba(52, 211, 153, 0.28);
  background: radial-gradient(
    circle at top,
    rgba(52, 211, 153, 0.16),
    rgba(2, 12, 23, 0.96)
  );
  box-shadow:
    0 0 40px rgba(52, 211, 153, 0.12),
    0 18px 50px rgba(0, 0, 0, 0.28);
}

.completion-panel h2 {
  color: #d1fae5;
}

.final-signoff {
  background: linear-gradient(90deg, #34d399, #6ee7b7, #a7f3d0);
  box-shadow:
    0 0 18px rgba(52, 211, 153, 0.35),
    0 0 48px rgba(52, 211, 153, 0.18);
}

.final-signoff:hover {
  transform: translateY(-2px) scale(1.01);
}
</style>
