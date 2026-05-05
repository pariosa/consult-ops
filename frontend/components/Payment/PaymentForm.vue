<template>
  <form class="form-shell payment" @submit.prevent="submit">
    <h3 class="form-title">Create Payment</h3>

    <div class="form-group grid">
      <label class="form-label">Invoice ID: </label>

      <input
        v-model.number="form.invoice_id"
        type="number"
        placeholder="Invoice ID"
        class="form-input"
      />
      <label class="form-label">Amount: </label>
      <input
        v-model.number="form.amount"
        type="number"
        placeholder="Amount"
        class="form-input"
      />
      <label class="form-label">Paid At: </label>

      <input v-model="form.paid_at" placeholder="Paid At" class="form-input" />
      <label class="form-label">Currency: </label>

      <input
        v-model="form.currency"
        placeholder="Currency"
        class="form-input"
      />
      <label class="form-label">Method: </label>

      <input v-model="form.method" placeholder="Method" class="form-input" />
      <label class="form-label">Reference: </label>

      <input
        v-model="form.reference"
        placeholder="Reference"
        class="form-input"
      />
      <label class="form-label">Amount: </label>

      <textarea
        v-model="form.notes"
        placeholder="Notes"
        class="form-textarea"
      />
    </div>

    <button class="form-button" type="submit">Save Payment</button>
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
@import '../../assets/css/forms.css';
.payment {
  /* background: #ecfeff; */
}
.payment::before {
  background: linear-gradient(135deg, #06b6d4, #3b82f6);
}
</style>
