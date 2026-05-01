<!-- frontend/components/LoginForm.vue -->
<template>
  <form
    @submit.prevent="submitForm"
    class="login-form"
    :class="`login-form--${userType}`"
  >
    <slot name="header">
      <div class="form-header">
        <p class="eyebrow">{{ eyebrow }}</p>
        <h2>{{ title }}</h2>
        <p class="subtitle">{{ subtitle }}</p>
      </div>
    </slot>

    <div class="form-group">
      <label for="email">Email</label>
      <input
        id="email"
        v-model="email"
        type="email"
        placeholder="Enter email"
        required
      />
    </div>

    <div class="form-group">
      <label for="password">Password</label>
      <input
        id="password"
        v-model="password"
        type="password"
        placeholder="Enter password"
        required
      />
    </div>

    <slot name="extra"></slot>

    <button type="submit">{{ submitText }}</button>

    <p v-if="error" class="error">{{ error }}</p>
  </form>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue';

const props = defineProps({
  userType: { type: String, default: 'consultant' },
  submitText: { type: String, default: 'Login' },
  initialEmail: { type: String, default: '' },
  initialPassword: { type: String, default: '' },
});

const emits = defineEmits<{
  (
    e: 'submit',
    payload: { email: string; password: string; userType: string },
  ): void;
}>();

const email = ref(props.initialEmail);
const password = ref(props.initialPassword);
const error = ref('');

const title = computed(() => {
  if (props.userType === 'client') return 'Client Portal Login';
  if (props.userType === 'admin') return 'Admin Console Login';
  return 'Consultant Workspace Login';
});

const eyebrow = computed(() => {
  if (props.userType === 'client') return 'Client Access';
  if (props.userType === 'admin') return 'Platform Admin';
  return 'Consult Ops';
});

const subtitle = computed(() => {
  if (props.userType === 'client')
    return 'Review projects, invoices, contracts, and payment status.';
  if (props.userType === 'admin')
    return 'Manage platform operations, users, and system workflows.';
  return 'Manage clients, projects, invoices, contracts, and payments.';
});

const submitForm = () => {
  if (!email.value || !password.value) {
    error.value = 'Both fields are required';
    return;
  }

  error.value = '';
  emits('submit', {
    email: email.value,
    password: password.value,
    userType: props.userType,
  });
};
</script>

<style scoped>
.login-form {
  width: min(100%, 420px);
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 2rem;
  border: 1px solid rgba(80, 210, 170, 0.35);
  border-radius: 1.25rem;
  background:
    linear-gradient(#08131f, #08131f) padding-box,
    linear-gradient(135deg, #3b82f6, #22c55e) border-box;
  color: #eef6ff;
  box-shadow: 0 24px 70px rgba(0, 0, 0, 0.35);
}

.form-header {
  margin-bottom: 0.5rem;
}

.eyebrow {
  margin: 0 0 0.25rem;
  color: #55d6be;
  font-size: 0.75rem;
  text-transform: uppercase;
  letter-spacing: 0.12em;
}

h2 {
  margin: 0;
  font-size: 1.5rem;
}

.subtitle {
  color: #9fb3c8;
  line-height: 1.5;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

input {
  border: 1px solid rgba(148, 163, 184, 0.35);
  border-radius: 0.75rem;
  padding: 0.75rem 0.85rem;
  background: #0f1d2b;
  color: white;
}

button {
  border: 0;
  border-radius: 0.85rem;
  padding: 0.85rem;
  color: #041016;
  font-weight: 700;
  cursor: pointer;
  background: linear-gradient(135deg, #60a5fa, #34d399);
}

.error {
  color: #f87171;
  margin: 0;
}
</style>
