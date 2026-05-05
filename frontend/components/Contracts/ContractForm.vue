<template>
  <form class="form-shell form-card contract" @submit.prevent="submit">
    <h3 class="form-title">Create Contract</h3>

    <div class="form-group grid">
      <label class="form-label">Title: </label>
      <input class="form-input" v-model="form.title" placeholder="Title" />
      <label class="form-label">Status: </label>

      <input class="form-input" v-model="form.status" placeholder="Status" />
      <label class="form-label">Start Date: </label>

      <input
        class="form-input"
        v-model="form.start_date"
        placeholder="Start Date"
      />
      <label class="form-label">End Date: </label>

      <input
        class="form-input"
        v-model="form.end_date"
        placeholder="End Date"
      />
      <label class="form-label">Signed At: </label>

      <input
        class="form-input"
        v-model="form.signed_at"
        placeholder="Signed At"
      />
      <label class="form-label">Value: </label>

      <input
        class="form-input"
        v-model.number="form.value"
        type="number"
        placeholder="Value"
      />
      <label class="form-label">Currency: </label>

      <input
        class="form-input"
        v-model="form.currency"
        placeholder="Currency"
      />
      <label class="form-label">External ID: </label>

      <input
        class="form-input"
        v-model="form.external_id"
        placeholder="External ID"
      />
      <label class="form-label">Notes: </label>

      <textarea
        class="form-textarea"
        v-model="form.notes"
        placeholder="Notes"
      />
    </div>

    <button class="form-button" type="submit">Save Contract</button>
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
@import '../../assets/css/forms.css';
.contract {
  /* background: #f0fdf4; */
}
.contract::before {
  background: linear-gradient(135deg, #22c55e, #3b82f6);
}
</style>
