<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useApi } from '~/composables/useApi';
import { useOperationalFinance } from '~/composables/useOperationalFinance';

const api = useApi();
const { getFinanceSummary, getPartyBalances } = useOperationalFinance();

const loading = ref(true);
const error = ref('');
const organization = ref<any>(null);
const summary = ref<any>(null);
const balances = ref<any[]>([]);

function money(cents: number) {
  return `$${((cents || 0) / 100).toFixed(2)}`;
}

const outstandingCents = computed(
  () =>
    (summary.value?.pending_cents || 0) +
    (summary.value?.processing_cents || 0),
);

async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    organization.value = await api.get('/api/me/organization');

    summary.value = await getFinanceSummary(organization.value.id);
    balances.value = await getPartyBalances(organization.value.id);
  } catch (err: any) {
    error.value = err?.message || 'Failed to load operational finance.';
  } finally {
    loading.value = false;
  }
}

onMounted(refresh);
</script>

<template>
  <DashboardShell
    title="Operational Finance"
    subtitle="Track obligations, balances, pending payouts, and paid transaction history."
  >
    <section v-if="loading" class="portal-section">
      Loading finance dashboard...
    </section>

    <section v-else-if="error" class="form-error">{{ error }}</section>

    <template v-else>
      <section class="finance-grid">
        <div class="finance-card">
          <p>Outstanding</p>
          <strong>{{ money(outstandingCents) }}</strong>
        </div>

        <div class="finance-card">
          <p>Pending</p>
          <strong>{{ money(summary.pending_cents) }}</strong>
        </div>

        <div class="finance-card">
          <p>Processing</p>
          <strong>{{ money(summary.processing_cents) }}</strong>
        </div>

        <div class="finance-card success">
          <p>Paid</p>
          <strong>{{ money(summary.paid_cents) }}</strong>
        </div>

        <div class="finance-card">
          <p>Total Obligations</p>
          <strong>{{ money(summary.total_obligations_cents) }}</strong>
        </div>
      </section>

      <section class="portal-section">
        <div class="section-header">
          <div>
            <p class="eyebrow">Party Balances</p>
            <h2>Who Owes / Who Is Owed</h2>
            <p>
              Pending and processing operational transactions grouped by
              verified party identity.
            </p>
          </div>

          <button class="form-button secondary" @click="refresh">
            Refresh
          </button>
        </div>

        <div v-if="!balances.length" class="empty-state">
          No party balances yet.
        </div>

        <div v-else class="balance-table">
          <div class="balance-row balance-row--header">
            <span>Party</span>
            <span>Type</span>
            <span>Payable</span>
            <span>Receivable</span>
            <span>Net</span>
          </div>

          <div
            v-for="party in balances"
            :key="party.party_id"
            class="balance-row"
          >
            <span>
              {{ party.party_name }}
              <small v-if="party.is_verified">Verified</small>
            </span>
            <span>{{ party.party_type }}</span>
            <span>{{ money(party.payable_cents) }}</span>
            <span>{{ money(party.receivable_cents) }}</span>
            <strong>{{ money(party.net_cents) }}</strong>
          </div>
        </div>
      </section>
    </template>
  </DashboardShell>
</template>

<style scoped>
.portal-section,
.finance-card {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 18px;
  background: linear-gradient(
    180deg,
    rgba(15, 23, 42, 0.96),
    rgba(2, 12, 23, 0.96)
  );
  color: #e5eefc;
  padding: 24px;
  margin-bottom: 24px;
}

.finance-grid {
  display: grid;
  gap: 14px;
  margin-bottom: 24px;
}

.finance-card p {
  color: #94a3b8;
  font-size: 0.8rem;
  font-weight: 900;
  letter-spacing: 0.08em;
  margin: 0 0 10px;
  text-transform: uppercase;
}

.finance-card strong {
  color: #f8fafc;
  font-size: 2rem;
}

.finance-card.success {
  border-color: rgba(52, 211, 153, 0.4);
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

h2 {
  color: #f8fafc;
  margin: 0 0 8px;
}

p {
  color: #cbd5e1;
}

.balance-table {
  display: grid;
  gap: 10px;
}

.balance-row {
  display: grid;
  grid-template-columns: 1.4fr 0.8fr 0.8fr 0.8fr 0.8fr;
  gap: 12px;
  align-items: center;
  border: 1px solid rgba(45, 212, 191, 0.18);
  border-radius: 14px;
  background: rgba(8, 31, 42, 0.86);
  padding: 14px;
}

.balance-row--header {
  background: rgba(2, 12, 23, 0.92);
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.1em;
  text-transform: uppercase;
}

.balance-row small {
  display: inline-flex;
  margin-left: 8px;
  border-radius: 999px;
  background: rgba(52, 211, 153, 0.16);
  color: #6ee7b7;
  font-size: 0.65rem;
  font-weight: 900;
  padding: 4px 8px;
  text-transform: uppercase;
}

.form-button.secondary {
  background: rgba(15, 23, 42, 0.95);
  border: 1px solid rgba(45, 212, 191, 0.32);
  border-radius: 12px;
  color: #e5eefc;
  cursor: pointer;
  font-weight: 800;
  padding: 12px 16px;
}

.form-error,
.empty-state {
  border: 1px dashed rgba(45, 212, 191, 0.28);
  border-radius: 14px;
  color: #cbd5e1;
  padding: 18px;
}

@media (min-width: 900px) {
  .finance-grid {
    grid-template-columns: repeat(5, minmax(0, 1fr));
  }
}

@media (max-width: 800px) {
  .balance-row {
    grid-template-columns: 1fr;
  }

  .balance-row--header {
    display: none;
  }
}
</style>
