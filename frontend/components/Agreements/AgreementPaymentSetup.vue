<!-- frontend/components/Agreements/AgreementPaymentSetup.vue -->

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { usePartyPaymentProfiles } from '~/composables/usePartyPaymentProfiles';
const props = defineProps<{
  parties: any[];
  payerPartyId?: number | null;
  payeePartyId?: number | null;
  triggerEvent?: string;
  percent?: number | null;
  amountCents?: number | null;
}>();

const emit = defineEmits<{
  refresh: [];
  readinessChange: [
    payload: {
      payerReady: boolean;
      payeeReady: boolean;
      allReady: boolean;
    },
  ];
}>();

const {
  getReadiness,
  upsertProfile,
  verifyParty,
  markPayoutReadyDev,
  markPayerAuthorizedDev,
} = usePartyPaymentProfiles();

const payerReadiness = ref<any>(null);
const payeeReadiness = ref<any>(null);
const loading = ref(false);
const error = ref('');
const success = ref('');
const hasPayerSelected = computed(() => Boolean(props.payerPartyId));
const hasPayeeSelected = computed(() => Boolean(props.payeePartyId));

const canRunPayerActions = computed(
  () => hasPayerSelected.value && !loading.value,
);

const canRunPayeeActions = computed(
  () => hasPayeeSelected.value && !loading.value,
);

const payer = computed(() =>
  props.parties.find(
    (party) => Number(party.id) === Number(props.payerPartyId),
  ),
);

const payee = computed(() =>
  props.parties.find(
    (party) => Number(party.id) === Number(props.payeePartyId),
  ),
);

const rulePreview = computed(() => {
  if (!payer.value || !payee.value) {
    return 'Select a payer and payee to preview the payout rule.';
  }

  const trigger = props.triggerEvent || 'MilestoneApproved';

  const amountText = props.amountCents
    ? `$${(Number(props.amountCents) / 100).toFixed(2)}`
    : `${props.percent || 100}% of the milestone amount`;

  return `When ${trigger} happens, ${payer.value.name} owes ${payee.value.name} ${amountText}.`;
});

const payerReady = computed(() => Boolean(payerReadiness.value?.payer_ready));
const payeeReady = computed(() => Boolean(payeeReadiness.value?.payee_ready));
const payerVerified = computed(() =>
  Boolean(payerReadiness.value?.is_verified),
);
const payeeVerified = computed(() =>
  Boolean(payeeReadiness.value?.is_verified),
);

const checklist = computed(() => [
  {
    label: 'Verified payer selected',
    complete: Boolean(payer.value && payerVerified.value),
  },
  {
    label: 'Payer funding authorized',
    complete: Boolean(payer.value && payerReady.value),
  },
  {
    label: 'Verified payee selected',
    complete: Boolean(payee.value && payeeVerified.value),
  },
  {
    label: 'Payee payout-ready',
    complete: Boolean(payee.value && payeeReady.value),
  },
]);

async function loadReadiness() {
  error.value = '';
  success.value = '';

  try {
    if (props.payerPartyId) {
      payerReadiness.value = await getReadiness(Number(props.payerPartyId));
    } else {
      payerReadiness.value = null;
    }

    if (props.payeePartyId) {
      payeeReadiness.value = await getReadiness(Number(props.payeePartyId));
    } else {
      payeeReadiness.value = null;
    }
  } catch (err: any) {
    error.value = err?.message || 'Failed to load payment readiness.';
  }
}

async function runAction(action: () => Promise<any>, message: string) {
  loading.value = true;
  error.value = '';
  success.value = '';

  try {
    await action();
    success.value = message;
    await loadReadiness();
    emit('refresh');
  } catch (err: any) {
    error.value = err?.message || 'Action failed.';
  } finally {
    loading.value = false;
  }
}

async function configurePayer() {
  if (!props.payerPartyId) return;

  await runAction(
    () =>
      upsertProfile(Number(props.payerPartyId), {
        payment_role: 'payer',
        payer_authorization_scope: 'agreement',
      }),
    'Payer profile configured.',
  );
}

async function configurePayee() {
  if (!props.payeePartyId) return;

  await runAction(
    () =>
      upsertProfile(Number(props.payeePartyId), {
        payment_role: 'payee',
      }),
    'Payee profile configured.',
  );
}

async function verifySelectedPayer() {
  if (!props.payerPartyId) return;
  await runAction(
    () => verifyParty(Number(props.payerPartyId)),
    'Payer verified.',
  );
}

async function verifySelectedPayee() {
  if (!props.payeePartyId) return;
  await runAction(
    () => verifyParty(Number(props.payeePartyId)),
    'Payee verified.',
  );
}

async function authorizePayerDev() {
  if (!props.payerPartyId) return;
  await runAction(
    () => markPayerAuthorizedDev(Number(props.payerPartyId)),
    'Payer marked authorized in dev mode.',
  );
}

async function markPayeeReadyDev() {
  if (!props.payeePartyId) return;
  await runAction(
    () => markPayoutReadyDev(Number(props.payeePartyId)),
    'Payee marked payout-ready in dev mode.',
  );
}

watch(() => [props.payerPartyId, props.payeePartyId], loadReadiness);
watch(
  [payerReady, payeeReady],
  () => {
    emit('readinessChange', {
      payerReady: payerReady.value,
      payeeReady: payeeReady.value,
      allReady: payerReady.value && payeeReady.value,
    });
  },
  { immediate: true },
);
onMounted(loadReadiness);
</script>

<template>
  <section class="payment-setup">
    <div class="section-header">
      <div>
        <p class="eyebrow">Payment Certainty</p>
        <h2>Verified payer and payee setup</h2>
        <p>
          Confirm who pays, who receives funds, and whether both sides are ready
          before attaching automatic payout rules.
        </p>
      </div>
    </div>

    <p v-if="error" class="form-error">{{ error }}</p>
    <p v-if="success" class="success-state">{{ success }}</p>

    <div class="readiness-grid">
      <article class="readiness-card">
        <p class="eyebrow">Payer / Funding Source</p>

        <h3>{{ payer?.name || 'No payer selected' }}</h3>
        <p>{{ payer?.email || 'Select payer from the rule form below.' }}</p>

        <div class="status-list">
          <span :class="['status-pill', payerVerified ? 'ready' : 'blocked']">
            {{ payerVerified ? 'Verified' : 'Not verified' }}
          </span>

          <span :class="['status-pill', payerReady ? 'ready' : 'blocked']">
            {{ payerReady ? 'Payment authorized' : 'Funding not authorized' }}
          </span>
        </div>

        <div class="actions">
          <button
            class="form-button secondary"
            :disabled="!canRunPayerActions"
            @click="verifySelectedPayer"
          >
            Verify Payer
          </button>

          <button
            class="form-button secondary"
            :disabled="!canRunPayerActions"
            @click="configurePayer"
          >
            Configure as Payer
          </button>

          <button
            class="form-button"
            :disabled="!canRunPayerActions"
            @click="authorizePayerDev"
          >
            Authorize Funding Dev
          </button>
        </div>
      </article>

      <article class="readiness-card">
        <p class="eyebrow">Payee / Recipient</p>

        <h3>{{ payee?.name || 'No payee selected' }}</h3>
        <p>{{ payee?.email || 'Select payee from the rule form below.' }}</p>

        <div class="status-list">
          <span :class="['status-pill', payeeVerified ? 'ready' : 'blocked']">
            {{ payeeVerified ? 'Verified' : 'Not verified' }}
          </span>

          <span :class="['status-pill', payeeReady ? 'ready' : 'blocked']">
            {{ payeeReady ? 'Payout ready' : 'Payout not ready' }}
          </span>
        </div>

        <div class="actions">
          <button
            class="form-button secondary"
            :disabled="!canRunPayeeActions"
            @click="verifySelectedPayee"
          >
            Verify Payee
          </button>

          <button
            class="form-button secondary"
            :disabled="!canRunPayeeActions"
            @click="configurePayee"
          >
            Configure as Payee
          </button>

          <button
            class="form-button"
            :disabled="!canRunPayeeActions"
            @click="markPayeeReadyDev"
          >
            Mark Payout Ready Dev
          </button>
        </div>
      </article>
    </div>

    <article class="preview-card">
      <p class="eyebrow">Rule Preview</p>
      <h3>{{ rulePreview }}</h3>

      <ul class="checklist">
        <li
          v-for="item in checklist"
          :key="item.label"
          :class="{ complete: item.complete }"
        >
          <span>{{ item.complete ? '✓' : '•' }}</span>
          {{ item.label }}
        </li>
      </ul>
    </article>
  </section>
</template>

<style scoped>
.payment-setup {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 18px;
  background: linear-gradient(
    180deg,
    rgba(15, 23, 42, 0.96),
    rgba(2, 12, 23, 0.96)
  );
  color: #e5eefc;
  padding: 24px;
}

.section-header {
  margin-bottom: 18px;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.12em;
  margin: 0 0 8px;
  text-transform: uppercase;
}

.readiness-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 16px;
}

.readiness-card,
.preview-card {
  border: 1px solid rgba(45, 212, 191, 0.16);
  border-radius: 16px;
  background: rgba(8, 31, 42, 0.7);
  padding: 18px;
}

.status-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin: 14px 0;
}

.status-pill {
  border-radius: 999px;
  font-size: 0.75rem;
  font-weight: 900;
  padding: 6px 10px;
}

.status-pill.ready {
  background: rgba(22, 163, 74, 0.18);
  border: 1px solid rgba(74, 222, 128, 0.32);
  color: #bbf7d0;
}

.status-pill.blocked {
  background: rgba(127, 29, 29, 0.22);
  border: 1px solid rgba(248, 113, 113, 0.32);
  color: #fecaca;
}

.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.preview-card {
  margin-top: 16px;
}

.checklist {
  display: grid;
  gap: 8px;
  list-style: none;
  margin: 16px 0 0;
  padding: 0;
}

.checklist li {
  color: #cbd5e1;
}

.checklist li.complete {
  color: #bbf7d0;
  font-weight: 800;
}

.form-button {
  background: linear-gradient(90deg, #60a5fa, #34d399);
  border: 0;
  border-radius: 12px;
  color: #020617;
  cursor: pointer;
  font-weight: 900;
  padding: 10px 13px;
}

.form-button.secondary {
  background: rgba(15, 23, 42, 0.95);
  border: 1px solid rgba(45, 212, 191, 0.32);
  color: #e5eefc;
}

.form-button:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

.form-error,
.success-state {
  border-radius: 14px;
  margin-bottom: 12px;
  padding: 14px;
}

.form-error {
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
}

.success-state {
  background: rgba(20, 83, 45, 0.24);
  color: #bbf7d0;
}

@media (max-width: 900px) {
  .readiness-grid {
    grid-template-columns: 1fr;
  }
}
</style>
