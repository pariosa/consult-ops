<template>
  <form class="form-shell form-card invoice" @submit.prevent="submit">
    <h3 class="form-title">Create Invoice</h3>

    <div class="form-group">
      <label class="form-label">Invoice ID: </label>
      <input class="form-input" v-model="form.id" placeholder="Invoice ID" />
      <label class="form-label">Amount: </label>
      <input
        class="form-input"
        v-model.number="form.amount"
        placeholder="Amount"
        type="number"
      />
      <label class="form-label">Status: </label>
      <input class="form-input" v-model="form.status" placeholder="Status" />
      <label class="form-label">Due Date: </label>
      <input
        class="form-input"
        v-model="form.due_date"
        placeholder="Due Date"
      />
      <label class="form-label">Subtotal: </label>
      <input
        v-model.number="form.subtotal"
        placeholder="Subtotal"
        type="number"
        class="form-input"
      />
    </div>
    <div class="form-group">
      <label class="form-label">Tax: </label>
      <input
        class="form-input"
        v-model.number="form.tax"
        placeholder="Tax"
        type="number"
      />
      <label class="form-label">Total: </label>
      <input
        class="form-input"
        v-model.number="form.total"
        placeholder="Total"
        type="number"
      />
      <label class="form-label">Currency: </label>
      <input
        class="form-input"
        v-model="form.currency"
        placeholder="Currency"
      />
      <label class="form-label">Notes: </label>
      <textarea
        class="form-textarea"
        v-model="form.notes"
        placeholder="Notes"
      />
    </div>
    <button class="form-button" type="submit">Save Invoice</button>
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
@import '../../assets/css/forms.css';
.invoice {
  /* background: #eff6ff; */
}
.invoice::before {
  background: linear-gradient(135deg, #22c55e, #3b82f6);
}
</style>
