<script setup lang="ts">
import TransactionStatusBadge from './TransactionStatusBadge.vue';

defineProps<{
  transactions: any[];
}>();

function formatMoney(cents: number, currency = 'usd') {
  return `${(cents / 100).toLocaleString(undefined, {
    style: 'currency',
    currency: currency.toUpperCase(),
  })}`;
}

function formatType(value: string) {
  return value?.replaceAll('_', ' ') || 'transaction';
}
</script>

<template>
  <section class="transaction-table-wrap">
    <div v-if="!transactions.length" class="empty-state">
      No operational transactions have been generated yet.
    </div>

    <div v-else class="transaction-table">
      <div class="transaction-row transaction-row--header">
        <span>Type</span>
        <span>From</span>
        <span>To</span>
        <span>Amount</span>
        <span>Status</span>
        <span>Trigger</span>
      </div>

      <div
        v-for="transaction in transactions"
        :key="transaction.id"
        class="transaction-row"
      >
        <span class="type-cell">
          {{ formatType(transaction.transaction_type) }}
        </span>

        <span>#{{ transaction.from_party_id }}</span>
        <span>#{{ transaction.to_party_id }}</span>

        <span class="amount-cell">
          {{ formatMoney(transaction.amount_cents, transaction.currency) }}
        </span>

        <span>
          <TransactionStatusBadge :status="transaction.status" />
        </span>

        <span>{{ transaction.trigger_event || 'Manual' }}</span>
      </div>
    </div>
  </section>
</template>

<style scoped>
.transaction-table-wrap {
  width: 100%;
}

.empty-state {
  border: 1px dashed rgba(45, 212, 191, 0.28);
  border-radius: 14px;
  color: #cbd5e1;
  padding: 18px;
}

.transaction-table {
  display: grid;
  gap: 10px;
}

.transaction-row {
  align-items: center;
  border: 1px solid rgba(45, 212, 191, 0.18);
  border-radius: 14px;
  background: rgba(8, 31, 42, 0.86);
  color: #cbd5e1;
  display: grid;
  gap: 12px;
  grid-template-columns: 1.3fr 0.7fr 0.7fr 0.9fr 0.9fr 1fr;
  padding: 14px;
}

.transaction-row--header {
  background: rgba(2, 12, 23, 0.92);
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.1em;
  text-transform: uppercase;
}

.type-cell {
  color: #f8fafc;
  font-weight: 800;
  text-transform: capitalize;
}

.amount-cell {
  color: #f8fafc;
  font-weight: 900;
}

@media (max-width: 860px) {
  .transaction-row {
    grid-template-columns: 1fr;
  }

  .transaction-row--header {
    display: none;
  }
}
</style>
