<template>
  <form @submit.prevent="submitForm" class="auth-form">
    <div class="form-header">
      <p class="eyebrow">Create Account</p>
      <h2>{{ title }}</h2>
      <p class="subtitle">{{ subtitle }}</p>
    </div>

    <div class="form-group">
      <label for="name">Name</label>
      <input
        id="name"
        v-model="name"
        type="text"
        placeholder="Enter name"
        required
      />
    </div>

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
        placeholder="Create password"
        required
      />
    </div>

    <div class="form-group">
      <label for="confirmPassword">Confirm password</label>
      <input
        id="confirmPassword"
        v-model="confirmPassword"
        type="password"
        placeholder="Confirm password"
        required
      />
    </div>

    <button type="submit">{{ submitText }}</button>

    <p v-if="localError" class="error">{{ localError }}</p>
    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="message" class="message">{{ message }}</p>
  </form>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';

const props = defineProps({
  userType: { type: String, default: 'consultant' },
  submitText: { type: String, default: 'Create account' },
  message: { type: String, default: '' },
  error: { type: String, default: '' },
});

const emit = defineEmits<{
  (
    e: 'submit',
    payload: {
      name: string;
      email: string;
      password: string;
      user_type: string;
    },
  ): void;
}>();

const name = ref('');
const email = ref('');
const password = ref('');
const confirmPassword = ref('');
const localError = ref('');

const title = computed(() => {
  if (props.userType === 'client') return 'Create a client portal account';
  if (props.userType === 'admin') return 'Create an admin account';
  return 'Create your consultant workspace';
});

const subtitle = computed(() => {
  if (props.userType === 'client')
    return 'Join your organization portal to view projects, invoices, and contracts.';
  if (props.userType === 'admin')
    return 'Create an internal platform account for user and organization management.';
  return 'Start managing clients, projects, contracts, invoices, and payments.';
});

const submitForm = () => {
  if (
    !name.value ||
    !email.value ||
    !password.value ||
    !confirmPassword.value
  ) {
    localError.value = 'All fields are required';
    return;
  }

  if (password.value !== confirmPassword.value) {
    localError.value = 'Passwords do not match';
    return;
  }

  localError.value = '';
  emit('submit', {
    name: name.value,
    email: email.value,
    password: password.value,
    user_type: props.userType,
  });
};
</script>

<style scoped>
.auth-form {
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
  box-shadow:
    0 20px 50px rgba(0, 0, 0, 0.35),
    0 0 25px rgba(16, 185, 129, 0.08);
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

input:-webkit-autofill {
  -webkit-box-shadow: 0 0 0 1000px #0f1d2b inset;
  -webkit-text-fill-color: white;
}

button {
  border: 0;
  border-radius: 0.85rem;
  padding: 0.85rem;
  color: #041016;
  font-weight: 700;
  cursor: pointer;
  background: linear-gradient(135deg, #60a5fa, #34d399);
  transition: all 0.2s ease;
}

button:hover {
  transform: translateY(-1px);
  filter: brightness(1.05);
}

.message {
  color: #6ee7b7;
}

.error {
  color: #f87171;
}
</style>
