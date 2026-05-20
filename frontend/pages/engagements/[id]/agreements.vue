<!-- frontend/pages/engagements/[id]/agreements.vue -->
<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import DashboardShell from '~/components/DashboardShell.vue';
import AgreementPaymentSetup from '~/components/Agreements/AgreementPaymentSetup.vue';

import { useApi } from '~/composables/useApi';
import { usePermissions } from '~/composables/usePermissions';
import { useAgreements } from '~/composables/useAgreements';

const route = useRoute();
const router = useRouter();

const api = useApi();

const {
  listOrganizationAgreements,
  createAgreement: createAgreementRequest,
  lockAgreement: lockAgreementRequest,
  listPayoutRules,
  createPayoutRule: createPayoutRuleRequest,
} = useAgreements();

const engagementId = Number(route.params.id);

const loading = ref(true);
const saving = ref(false);
const error = ref('');

const engagement = ref<any>(null);
const organizationId = ref<number | null>(null);

const parties = ref<any[]>([]);
const agreements = ref<any[]>([]);
const payoutRules = ref<any[]>([]);

const selectedAgreementId = ref<number | null>(null);

const { canManageAgreements } = usePermissions();

const setupPayerPartyId = ref<number | null>(null);
const setupPayeePartyId = ref<number | null>(null);

const paymentReadiness = ref({
  payerReady: false,
  payeeReady: false,
  allReady: false,
});

const clientParty = ref({
  name: '',
  email: '',
  party_type: 'client',
});

const contractorParty = ref({
  name: '',
  email: '',
  party_type: 'contractor',
});

const agreementForm = ref({
  title: 'Client pays contractor on milestone approval',
  agreement_type: 'milestone_payout',
});

const payoutRuleForm = ref({
  from_party_id: null as number | null,
  to_party_id: null as number | null,
  rule_type: 'contractor_payout',
  percent: 100,
  amount_cents: null as number | null,
  trigger_event: 'MilestoneApproved',
});

const selectedAgreement = computed(() =>
  agreements.value.find(
    (agreement) => agreement.id === selectedAgreementId.value,
  ),
);

const agreementLocked = computed(
  () => selectedAgreement.value?.status === 'locked',
);

const verifiedParties = computed(() =>
  parties.value.filter((party) => Number(party.is_verified) === 1),
);

const verifiedClientParties = computed(() =>
  verifiedParties.value.filter((party) => party.party_type === 'client'),
);

const verifiedContractorParties = computed(() =>
  verifiedParties.value.filter((party) =>
    ['contractor', 'subcontractor'].includes(party.party_type),
  ),
);

const clientPayerParties = computed(() =>
  parties.value.filter((party) => party.party_type === 'client'),
);

const contractorPayeeParties = computed(() =>
  parties.value.filter((party) =>
    ['contractor', 'subcontractor'].includes(party.party_type),
  ),
);

function updatePaymentReadiness(payload: any) {
  paymentReadiness.value = payload;

  if (payload.payerReady && setupPayerPartyId.value) {
    payoutRuleForm.value.from_party_id = setupPayerPartyId.value;
  }

  if (payload.payeeReady && setupPayeePartyId.value) {
    payoutRuleForm.value.to_party_id = setupPayeePartyId.value;
  }
}

function preparePayer(partyId: number) {
  setupPayerPartyId.value = Number(partyId);

  paymentReadiness.value = {
    ...paymentReadiness.value,
    payerReady: false,
    allReady: false,
  };
}

function preparePayee(partyId: number) {
  setupPayeePartyId.value = Number(partyId);

  paymentReadiness.value = {
    ...paymentReadiness.value,
    payeeReady: false,
    allReady: false,
  };
}

function partyLabel(partyId: number) {
  const party = parties.value.find((item) => item.id === partyId);

  if (!party) return `Party #${partyId}`;

  const verified = Number(party.is_verified) === 1 ? 'Verified' : 'Unverified';

  return `${party.name} — ${verified} ${party.party_type}`;
}

function formatPercent(rule: any) {
  if (rule.percent) return `${rule.percent}%`;

  if (rule.amount_cents) {
    return `$${(rule.amount_cents / 100).toFixed(2)}`;
  }

  return 'Not configured';
}

async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    engagement.value = await api.get(`/api/engagements/${engagementId}`);

    organizationId.value = engagement.value.organization_id;

    parties.value = await api.get(
      `/api/organizations/${organizationId.value}/parties`,
    );

    agreements.value = await listOrganizationAgreements(organizationId.value);

    const engagementAgreements = agreements.value.filter(
      (agreement: any) => agreement.engagement_id === engagementId,
    );

    if (engagementAgreements.length) {
      selectedAgreementId.value = engagementAgreements[0].id;

      payoutRules.value = await listPayoutRules(selectedAgreementId.value);
    } else {
      selectedAgreementId.value = null;
      payoutRules.value = [];
    }
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to load agreement setup.';
  } finally {
    loading.value = false;
  }
}

async function createParty(payload: any) {
  if (!organizationId.value) return;

  saving.value = true;
  error.value = '';

  try {
    await api.post(
      `/api/organizations/${organizationId.value}/parties`,
      payload,
    );

    await refresh();
  } catch (err: any) {
    error.value = err?.data?.error || err?.message || 'Failed to create party.';
  } finally {
    saving.value = false;
  }
}

async function createClientParty() {
  await createParty(clientParty.value);

  clientParty.value = {
    name: '',
    email: '',
    party_type: 'client',
  };
}

async function createContractorParty() {
  await createParty(contractorParty.value);

  contractorParty.value = {
    name: '',
    email: '',
    party_type: 'contractor',
  };
}

async function createVerifiedClientParty() {
  if (!organizationId.value || !engagement.value?.client_id) {
    error.value = 'No client linked to engagement.';
    return;
  }

  saving.value = true;
  error.value = '';

  try {
    await api.post(
      `/api/organizations/${organizationId.value}/parties/from-client/${engagement.value.client_id}`,
      {},
    );

    await refresh();
  } catch (err: any) {
    error.value =
      err?.data?.error ||
      err?.message ||
      'Failed to create verified client party.';
  } finally {
    saving.value = false;
  }
}

async function createVerifiedUserParty() {
  if (!organizationId.value) return;

  saving.value = true;
  error.value = '';

  try {
    await api.post(
      `/api/organizations/${organizationId.value}/parties/from-user/${engagement.value?.contractor_user_id}`,
      {},
    );

    await refresh();
  } catch (err: any) {
    error.value =
      err?.data?.error ||
      err?.message ||
      'Failed to create verified contractor party.';
  } finally {
    saving.value = false;
  }
}

async function createAgreement() {
  if (!organizationId.value) return;

  saving.value = true;
  error.value = '';

  try {
    const agreement = await createAgreementRequest(organizationId.value, {
      engagement_id: engagementId,
      title: agreementForm.value.title,
      agreement_type: agreementForm.value.agreement_type,
    });

    selectedAgreementId.value = agreement.id;

    await refresh();
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to create agreement.';
  } finally {
    saving.value = false;
  }
}

async function createPayoutRule() {
  if (!selectedAgreementId.value) {
    error.value = 'Create an agreement before adding payout rules.';
    return;
  }

  if (
    !payoutRuleForm.value.from_party_id ||
    !payoutRuleForm.value.to_party_id
  ) {
    error.value = 'Select both payer and payee.';
    return;
  }

  saving.value = true;
  error.value = '';

  try {
    await createPayoutRuleRequest(selectedAgreementId.value, {
      from_party_id: payoutRuleForm.value.from_party_id,
      to_party_id: payoutRuleForm.value.to_party_id,
      rule_type: payoutRuleForm.value.rule_type,
      percent: payoutRuleForm.value.percent,
      amount_cents: payoutRuleForm.value.amount_cents,
      trigger_event: payoutRuleForm.value.trigger_event,
    });

    payoutRules.value = await listPayoutRules(selectedAgreementId.value);
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to create payout rule.';
  } finally {
    saving.value = false;
  }
}

async function lockAgreement() {
  if (!selectedAgreementId.value) return;

  saving.value = true;
  error.value = '';

  try {
    await lockAgreementRequest(selectedAgreementId.value);

    await refresh();
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to lock agreement.';
  } finally {
    saving.value = false;
  }
}

function goToTransactions() {
  router.push(`/engagements/${engagementId}/transactions`);
}

onMounted(refresh);
</script>

<template>
  <DashboardShell
    title="Agreement Rules"
    subtitle="Configure constrained payer/payee rules for this engagement."
  >
    <section v-if="loading" class="portal-section">
      Loading agreement setup...
    </section>

    <section v-else-if="error" class="form-error">
      {{ error }}
    </section>

    <template v-else>
      <section class="portal-section hero-section">
        <p class="eyebrow">Operational Agreement</p>

        <h2>{{ engagement?.title }}</h2>

        <p>
          Define who pays whom, under which agreement, and what workflow event
          generates the transaction.
        </p>

        <button class="form-button secondary" @click="goToTransactions">
          View Transaction Ledger
        </button>
      </section>

      <AgreementPaymentSetup
        v-if="canManageAgreements"
        :parties="parties"
        :trigger-event="payoutRuleForm.trigger_event"
        :percent="Number(payoutRuleForm.percent || 0)"
        :payer-party-id="setupPayerPartyId"
        :payee-party-id="setupPayeePartyId"
        :amount-cents="
          payoutRuleForm.amount_cents
            ? Number(payoutRuleForm.amount_cents)
            : null
        "
        @refresh="refresh"
        @readiness-change="updatePaymentReadiness"
      />

      <section v-if="canManageAgreements" class="portal-section">
        <p class="eyebrow">Current Setup Selection</p>

        <p>
          Payer being prepared:
          <strong>
            {{
              setupPayerPartyId
                ? partyLabel(setupPayerPartyId)
                : 'None selected'
            }}
          </strong>
        </p>

        <p>
          Payee being prepared:
          <strong>
            {{
              setupPayeePartyId
                ? partyLabel(setupPayeePartyId)
                : 'None selected'
            }}
          </strong>
        </p>
      </section>

      <section v-if="canManageAgreements" class="setup-grid">
        <div class="portal-section">
          <p class="eyebrow">Client Party</p>

          <h2>Create Client</h2>

          <label>Name</label>

          <input v-model="clientParty.name" class="form-input" />

          <label>Email</label>

          <input v-model="clientParty.email" class="form-input" />

          <button
            class="form-button"
            :disabled="saving || !clientParty.name"
            @click="createClientParty"
          >
            Add Client Party
          </button>
        </div>

        <div class="portal-section">
          <p class="eyebrow">Contractor Party</p>

          <h2>Create Contractor</h2>

          <label>Name</label>

          <input v-model="contractorParty.name" class="form-input" />

          <label>Email</label>

          <input v-model="contractorParty.email" class="form-input" />

          <button
            class="form-button"
            :disabled="saving || !contractorParty.name"
            @click="createContractorParty"
          >
            Add Contractor Party
          </button>
        </div>
      </section>

      <section v-if="!canManageAgreements" class="form-error">
        You can view this engagement, but only admins, operations managers,
        finance admins, or payment moderators can manage agreement rules.
      </section>

      <section class="portal-section">
        <p class="eyebrow">Party Registry</p>

        <h2>Available Payers and Payees</h2>

        <div class="setup-grid">
          <div>
            <h3>Client / Payer Parties</h3>

            <div
              v-for="party in clientPayerParties"
              :key="party.id"
              class="ops-card"
            >
              <strong>{{ party.name }}</strong>

              <p>{{ party.email }}</p>

              <p>{{ partyLabel(party.id) }}</p>

              <button
                class="form-button secondary"
                @click="preparePayer(party.id)"
              >
                Prepare as Payer
              </button>
            </div>
          </div>

          <div>
            <h3>Contractor / Payee Parties</h3>

            <div
              v-for="party in contractorPayeeParties"
              :key="party.id"
              class="ops-card"
            >
              <strong>{{ party.name }}</strong>

              <p>{{ party.email }}</p>

              <p>{{ partyLabel(party.id) }}</p>

              <button
                class="form-button secondary"
                @click="preparePayee(party.id)"
              >
                Prepare as Payee
              </button>
            </div>
          </div>
        </div>
      </section>

      <section class="portal-section">
        <p class="eyebrow">Agreement</p>

        <h2>Engagement Agreement</h2>

        <div v-if="selectedAgreement" class="ops-card">
          <h3>{{ selectedAgreement.title }}</h3>

          <p>Type: {{ selectedAgreement.agreement_type }}</p>

          <p>Status: {{ selectedAgreement.status }}</p>
        </div>

        <div v-if="agreementLocked" class="success-state">
          Agreement locked. Payout rules are protected and milestone approvals
          can now generate transactions.
        </div>

        <button
          v-if="canManageAgreements && selectedAgreement && !agreementLocked"
          class="form-button"
          :disabled="saving || !payoutRules.length"
          @click="lockAgreement"
        >
          Lock Agreement
        </button>

        <div v-if="!selectedAgreement" class="form-grid">
          <label>Agreement Title</label>

          <input v-model="agreementForm.title" class="form-input" />

          <label>Agreement Type</label>

          <select v-model="agreementForm.agreement_type" class="form-input">
            <option value="milestone_payout">Milestone Payout</option>

            <option value="subcontractor_split">Subcontractor Split</option>

            <option value="revenue_share">Revenue Share</option>

            <option value="dividend_share">Dividend Share</option>
          </select>

          <button
            v-if="canManageAgreements"
            class="form-button"
            :disabled="saving"
            @click="createAgreement"
          >
            Create Agreement
          </button>
        </div>
      </section>

      <section class="portal-section">
        <p class="eyebrow">Payout Rule</p>

        <h2>Generate Transactions On Approval</h2>

        <div class="form-grid">
          <label for="payer-party">Payer</label>

          <select
            id="payer-party"
            v-model.number="payoutRuleForm.from_party_id"
            class="form-input"
            :disabled="agreementLocked"
          >
            <option :value="null">Select payer</option>

            <option
              v-for="party in verifiedClientParties"
              :key="party.id"
              :value="party.id"
            >
              {{ party.name }} — Verified {{ party.party_type }}
            </option>
          </select>

          <label for="payee-party">Payee</label>

          <select
            id="payee-party"
            v-model.number="payoutRuleForm.to_party_id"
            class="form-input"
            :disabled="agreementLocked"
          >
            <option :value="null">Select payee</option>

            <option
              v-for="party in verifiedContractorParties"
              :key="party.id"
              :value="party.id"
            >
              {{ party.name }} — Verified {{ party.party_type }}
            </option>
          </select>

          <label>Rule Type</label>

          <select
            v-model="payoutRuleForm.rule_type"
            class="form-input"
            :disabled="agreementLocked"
          >
            <option value="contractor_payout">Contractor Payout</option>

            <option value="subcontractor_payout">Subcontractor Payout</option>

            <option value="revenue_share">Revenue Share</option>

            <option value="dividend">Dividend</option>
          </select>

          <label>Percent</label>

          <input
            v-model.number="payoutRuleForm.percent"
            type="number"
            min="1"
            max="100"
            class="form-input"
            :disabled="agreementLocked"
          />

          <label>Trigger</label>

          <select
            v-model="payoutRuleForm.trigger_event"
            class="form-input"
            :disabled="agreementLocked"
          >
            <option value="MilestoneApproved">Milestone Approved</option>

            <option value="EngagementCompleted">Engagement Completed</option>
          </select>

          <button
            v-if="canManageAgreements"
            class="form-button"
            :disabled="
              saving ||
              agreementLocked ||
              !selectedAgreementId ||
              !paymentReadiness.allReady
            "
            @click="createPayoutRule"
          >
            Add Payout Rule
          </button>

          <div
            v-if="canManageAgreements && !paymentReadiness.allReady"
            class="form-error"
          >
            Payment setup is incomplete. Verify payer funding and payee payout
            readiness before adding this payout rule.
          </div>
        </div>
      </section>

      <section v-if="canManageAgreements" class="portal-section">
        <p class="eyebrow">Verified Parties</p>

        <h2>Assign Verified Parties</h2>

        <div class="verified-grid">
          <button
            class="form-button secondary"
            @click="createVerifiedClientParty"
          >
            Create Verified Client Party
          </button>

          <button
            class="form-button secondary"
            :disabled="!engagement?.contractor_user_id"
            @click="createVerifiedUserParty"
          >
            Create Verified Contractor Party
          </button>
        </div>
      </section>

      <section class="portal-section">
        <p class="eyebrow">Existing Rules</p>

        <h2>Payout Rules</h2>

        <div v-if="!payoutRules.length" class="empty-state">
          No payout rules configured yet.
        </div>

        <div
          v-for="rule in payoutRules"
          :key="rule.id"
          class="ops-card rule-card"
        >
          <div>
            <h3>{{ rule.rule_type }}</h3>

            <p>Trigger: {{ rule.trigger_event }}</p>

            <p>Amount: {{ formatPercent(rule) }}</p>
          </div>

          <div>
            <p>From: {{ partyLabel(rule.from_party_id) }}</p>

            <p>To: {{ partyLabel(rule.to_party_id) }}</p>
          </div>
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

.setup-grid {
  display: grid;
  gap: 18px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 20px;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 800;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

h2,
h3 {
  color: #f8fafc;
  margin: 0 0 10px;
}

p,
label {
  color: #cbd5e1;
}

.form-grid {
  display: grid;
  gap: 12px;
}

.form-input {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 12px;
  background: rgba(2, 12, 23, 0.95);
  color: #f8fafc;
  padding: 12px 14px;
}

.form-button {
  border: 0;
  border-radius: 12px;
  background: linear-gradient(90deg, #60a5fa, #34d399);
  color: #04111f;
  cursor: pointer;
  font-weight: 800;
  padding: 12px 16px;
}

.form-button.secondary {
  background: rgba(15, 23, 42, 0.95);
  border: 1px solid rgba(45, 212, 191, 0.32);
  color: #e5eefc;
}

.form-button:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.form-error {
  border: 1px solid rgba(248, 113, 113, 0.38);
  border-radius: 14px;
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
  padding: 16px;
}

.success-state {
  border: 1px solid rgba(52, 211, 153, 0.28);
  border-radius: 14px;
  background: rgba(20, 83, 45, 0.24);
  color: #bbf7d0;
  padding: 16px;
  margin-top: 16px;
}

.ops-card {
  border: 1px solid rgba(45, 212, 191, 0.18);
  border-radius: 14px;
  background: rgba(8, 31, 42, 0.86);
  padding: 16px;
}

.rule-card {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  margin-top: 12px;
}

.empty-state {
  border: 1px dashed rgba(45, 212, 191, 0.28);
  border-radius: 14px;
  color: #cbd5e1;
  padding: 18px;
}

@media (min-width: 860px) {
  .setup-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
</style>
