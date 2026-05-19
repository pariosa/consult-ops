<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useOperationalTransactions } from '~/composables/useOperationalTransactions';

const {
  getOrganizationTransactions,
  markProcessing,
  markPaid,
  markFailed,
  cancelTransaction,
} = useOperationalTransactions();

const organizationId = 1;
const transactions = ref<any[]>([]);
const loading = ref(false);
const error = ref('');

const formatMoney = (cents: number, currency = 'usd') =>
  new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: currency.toUpperCase(),
  }).format((cents || 0) / 100);

async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    transactions.value = await getOrganizationTransactions(organizationId);
  } catch (err: any) {
    error.value = err?.message || 'Failed to load transactions.';
  } finally {
    loading.value = false;
  }
}

async function runAction(action: () => Promise<any>) {
  await action();
  await refresh();
}

onMounted(refresh);
</script>

<template>
  <DashboardShell
    title="Organization Transactions"
    subtitle="Review payout obligations, settlement status, and failed transfers."
  >
    <section class="portal-section">
      <p v-if="loading">Loading transactions...</p>
      <p v-else-if="error" class="form-error">{{ error }}</p>

      <div v-else-if="!transactions.length" class="empty-state">
        No transactions yet.
      </div>

      <div v-else class="table-list">
        <div class="table-row table-row--header">
          <span>Amount</span>
          <span>Status</span>
          <span>From</span>
          <span>To</span>
          <span>Milestone</span>
          <span>Actions</span>
        </div>

        <div v-for="tx in transactions" :key="tx.id" class="table-row">
          <span>{{ formatMoney(tx.amount_cents, tx.currency) }}</span>
          <span class="status-pill">{{ tx.status }}</span>
          <span>#{{ tx.from_party_id }}</span>
          <span>#{{ tx.to_party_id }}</span>
          <span>{{ tx.milestone_id ? `#${tx.milestone_id}` : '—' }}</span>

          <span class="actions">
            <button
              v-if="tx.status === 'pending'"
              @click="runAction(() => markProcessing(tx.id))"
            >
              Processing
            </button>

            <button
              v-if="['pending', 'processing'].includes(tx.status)"
              @click="runAction(() => markPaid(tx.id))"
            >
              Paid
            </button>

            <button
              v-if="['pending', 'processing'].includes(tx.status)"
              @click="runAction(() => markFailed(tx.id))"
            >
              Failed
            </button>

            <button
              v-if="['pending', 'processing'].includes(tx.status)"
              @click="runAction(() => cancelTransaction(tx.id))"
            >
              Cancel
            </button>
          </span>
        </div>
      </div>
    </section>
  </DashboardShell>
</template>
