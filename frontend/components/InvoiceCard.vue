<template>
  <div class="card">
    <h3>Invoice #{{ invoice.id }}</h3>
    <p>Amount: {{ formatCurrency(invoice.amount) }}</p>
    <p>Status: {{ invoice.status }}</p>
    <p>Due: {{ invoice.due_date }}</p>
  </div>
</template>

<script setup lang="ts">
defineProps<{ invoice: any }>();
const formatCurrency = (amount: number) => `$${amount.toFixed(2)}`;
</script>

<style scoped>
.card {
  border: 1px solid #ccc;
  border-radius: 8px;
  padding: 1rem;
  margin: 0.5rem 0;
  background: #fdfdfd;
}
</style>
