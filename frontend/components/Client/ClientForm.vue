<template>
  <form class="form-shell form-card client" @submit.prevent="submit">
    <h3 class="form-title">Create Client</h3>

    <div class="grid form-group">
      <label class="form-label">Client Name</label>
      <input v-model="form.name" class="form-input" required />

      <label class="form-label">Email</label>
      <input v-model="form.email" type="email" class="form-input" required />

      <label class="form-label">Company Name</label>
      <input v-model="form.company_name" class="form-input" />

      <label class="form-label">Phone</label>
      <input v-model="form.phone" class="form-input" />

      <label class="form-label">Tax ID</label>
      <input v-model="form.tax_id" class="form-input" />

      <label class="form-label">Address</label>
      <input v-model="form.address" class="form-input" />

      <label class="form-label">City</label>
      <input v-model="form.city" class="form-input" />

      <label class="form-label">State</label>
      <input v-model="form.state" class="form-input" />

      <label class="form-label">Zip</label>
      <input v-model="form.zip" class="form-input" />

      <label class="form-label">Country</label>
      <input v-model="form.country" class="form-input" />
    </div>

    <button class="form-button" type="submit">Save Client</button>
  </form>
</template>

<script setup lang="ts">
import { reactive } from 'vue';

const emit = defineEmits<{
  submit: [payload: any];
}>();

const form = reactive({
  name: '',
  email: '',
  tax_id: '',
  phone: '',
  company_name: '',
  address: '',
  city: '',
  state: '',
  zip: '',
  country: 'US',
});

function submit() {
  emit('submit', { ...form });
}
</script>

<style scoped>
@import '../../assets/css/forms.css';

.client::before {
  background: linear-gradient(135deg, #14b8a6, #22c55e);
}
</style>
