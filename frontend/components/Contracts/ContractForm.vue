<template>
  <form class="form-card contract" @submit.prevent="submit">
    <h3>Create Contract</h3>

    <div class="grid">
      <input v-model="form.title" placeholder="Title" />
      <input v-model="form.status" placeholder="Status" />

      <input v-model="form.start_date" placeholder="Start Date" />
      <input v-model="form.end_date" placeholder="End Date" />
      <input v-model="form.signed_at" placeholder="Signed At" />

      <input v-model.number="form.value" type="number" placeholder="Value" />
      <input v-model="form.currency" placeholder="Currency" />

      <input v-model="form.external_id" placeholder="External ID" />
      <textarea v-model="form.notes" placeholder="Notes" />
    </div>

    <button type="submit">Save Contract</button>
  </form>
</template>

<script setup lang="ts">
import { reactive } from 'vue';

const form = reactive({
  title: '',
  status: '',
  start_date: '',
  end_date: '',
  signed_at: '',
  value: 0,
  currency: '',
  external_id: '',
  notes: '',
});

const submit = () => console.log('Contract:', form);
</script>

<style scoped>
@import '../form.css';
.contract {
  background: #f0fdf4;
}
.contract::before {
  background: linear-gradient(135deg, #22c55e, #3b82f6);
}
</style>
