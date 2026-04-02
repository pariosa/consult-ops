<!-- src/components/LoginForm.vue -->
<template>
  <form @submit.prevent="submitForm" class="login-form">
    <slot name="header">
      <h2>Login</h2>
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
import { ref, defineProps, defineEmits } from 'vue';

const props = defineProps({
  submitText: { type: String, default: 'Login' },
  initialEmail: { type: String, default: '' },
  initialPassword: { type: String, default: '' },
});

const emits = defineEmits<{
  (e: 'submit', payload: { email: string; password: string }): void;
}>();

const email = ref(props.initialEmail);
const password = ref(props.initialPassword);
const error = ref('');

const submitForm = () => {
  if (!email.value || !password.value) {
    error.value = 'Both fields are required';
    return;
  }
  error.value = '';
  emits('submit', { email: email.value, password: password.value });
};
</script>

<style scoped>
.login-form {
  display: flex;
  flex-direction: column;
  width: 300px;
}
.form-group {
  margin-bottom: 1rem;
}
.error {
  color: red;
  margin-top: 0.5rem;
}
button {
  padding: 0.5rem;
}
</style>