<template>
  <form @submit.prevent="submitForm" class="auth-form">
    <div class="form-header">
      <p class="eyebrow">Account Recovery</p>
      <h2>Reset your password</h2>
      <p class="subtitle">
        Enter your email and we’ll generate a reset link for your account.
      </p>
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

    <button type="submit">{{ submitText }}</button>

    <p v-if="message" class="message">{{ message }}</p>
    <p v-if="error" class="error">{{ error }}</p>
  </form>
</template>

<script setup lang="ts">
import { ref } from 'vue';

defineProps({
  submitText: { type: String, default: 'Send reset link' },
  message: { type: String, default: '' },
  error: { type: String, default: '' },
});

const emits = defineEmits<{
  (e: 'submit', payload: { email: string }): void;
}>();

const email = ref('');

const submitForm = () => {
  emits('submit', { email: email.value });
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

button {
  border: 0;
  border-radius: 0.85rem;
  padding: 0.85rem;
  color: #041016;
  font-weight: 700;
  cursor: pointer;
  background: linear-gradient(135deg, #60a5fa, #34d399);
}

input:-webkit-autofill {
  -webkit-box-shadow: 0 0 0 1000px #0f1d2b inset;
  -webkit-text-fill-color: white;
}

.message {
  color: #6ee7b7;
}

.error {
  color: #f87171;
}
</style>
