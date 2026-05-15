<script setup lang="ts">
import { onMounted, ref } from 'vue';
import OperationalTransactionTable from '~/components/Transactions/OperationalTransactionTable.vue';
import { useOperationalTransactions } from '~/composables/useOperationalTransactions';

const route = useRoute();
const engagementId = Number(route.params.id);

const transactions = ref<any[]>([]);
const loading = ref(true);
const error = ref('');

const {
  getEngagementTransactions,
  markProcessing,
  markPaid,
  markFailed,
  cancelTransaction,
} = useOperationalTransactions();

async function applyTransactionAction(action: () => Promise<any>) {
  error.value = '';

  try {
    await action();
    await refresh();
  } catch (err: any) {
    error.value = err?.message || 'Failed to update transaction.';
  }
}
async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    transactions.value = await getEngagementTransactions(engagementId);
  } catch (err: any) {
    error.value = err?.message || 'Failed to load transactions.';
  } finally {
    loading.value = false;
  }
}

onMounted(refresh);
</script>

<template>
  <DashboardShell
    title="Operational Transactions"
    subtitle="Review payer/payee obligations generated from agreements and workflow events."
  >
    <section v-if="loading" class="portal-section">
      Loading transactions...
    </section>

    <section v-else-if="error" class="form-error">
      {{ error }}
    </section>

    <section v-else class="portal-section">
      <div class="section-header">
        <div>
          <p class="eyebrow">Agreement-Linked Transactions</p>
          <h2>Engagement #{{ engagementId }}</h2>
          <p>
            These transactions represent structured payout obligations created
            by agreement rules.
          </p>
        </div>

        <button class="form-button secondary" @click="refresh">Refresh</button>
      </div>

      <OperationalTransactionTable
        :transactions="transactions"
        @mark-processing="
          (tx) => applyTransactionAction(() => markProcessing(tx.id))
        "
        @mark-paid="(tx) => applyTransactionAction(() => markPaid(tx.id))"
        @mark-failed="(tx) => applyTransactionAction(() => markFailed(tx.id))"
        @cancel="(tx) => applyTransactionAction(() => cancelTransaction(tx.id))"
      />
    </section>
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

.section-header {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  align-items: flex-start;
  margin-bottom: 20px;
}

.section-header h2 {
  color: #f8fafc;
  margin: 0 0 8px;
}

.section-header p {
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

.form-button {
  border: 0;
  border-radius: 12px;
  cursor: pointer;
  font-weight: 800;
  padding: 12px 16px;
}

.form-button.secondary {
  background: rgba(15, 23, 42, 0.95);
  border: 1px solid rgba(45, 212, 191, 0.32);
  color: #e5eefc;
}

.form-error {
  border: 1px solid rgba(248, 113, 113, 0.38);
  border-radius: 14px;
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
  padding: 16px;
}
</style>
