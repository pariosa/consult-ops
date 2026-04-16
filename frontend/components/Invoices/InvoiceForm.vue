<template>
  <form class="form-card invoice" @submit.prevent="submit">
    <h3>Create Invoice</h3>

    <div class="grid">
      <input v-model="form.id" placeholder="Invoice ID" />
      <input v-model.number="form.amount" placeholder="Amount" type="number" />
      <input v-model="form.status" placeholder="Status" />
      <input v-model="form.due_date" placeholder="Due Date" />

      <input
        v-model.number="form.subtotal"
        placeholder="Subtotal"
        type="number"
      />
      <input v-model.number="form.tax" placeholder="Tax" type="number" />
      <input v-model.number="form.total" placeholder="Total" type="number" />
      <input v-model="form.currency" placeholder="Currency" />

      <textarea v-model="form.notes" placeholder="Notes" />
    </div>

    <button type="submit">Save Invoice</button>
  </form>
</template>

<script setup lang="ts">
import { reactive } from 'vue';

const form = reactive({
  id: '',
  amount: 0,
  status: '',
  due_date: '',
  subtotal: 0,
  tax: 0,
  total: 0,
  currency: 'USD',
  notes: '',
});

const submit = () => {
  console.log('Invoice:', form);
};
</script>

<style scoped>
@import '../form.css';
.invoice {
  background: #eff6ff;
}
.invoice::before {
  background: linear-gradient(135deg, #22c55e, #3b82f6);
}
</style>
