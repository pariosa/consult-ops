<template>
  <form class="form-card payment" @submit.prevent="submit">
    <h3>Create Payment</h3>

    <div class="grid">
      <input
        v-model.number="form.invoice_id"
        type="number"
        placeholder="Invoice ID"
      />
      <input v-model.number="form.amount" type="number" placeholder="Amount" />
      <input v-model="form.paid_at" placeholder="Paid At" />

      <input v-model="form.currency" placeholder="Currency" />
      <input v-model="form.method" placeholder="Method" />
      <input v-model="form.reference" placeholder="Reference" />

      <textarea v-model="form.notes" placeholder="Notes" />
    </div>

    <button type="submit">Save Payment</button>
  </form>
</template>

<script setup lang="ts">
import { reactive } from 'vue';

const form = reactive({
  invoice_id: 0,
  amount: 0,
  paid_at: '',
  currency: '',
  method: '',
  reference: '',
  notes: '',
});

const submit = () => console.log('Payment:', form);
</script>

<style scoped>
@import '../form.css';
.payment {
  background: #ecfeff;
}
.payment::before {
  background: linear-gradient(135deg, #06b6d4, #3b82f6);
}
</style>
