<script setup lang="ts">
import { ref } from 'vue';
import ForgotPasswordForm from '~/components/ForgotPasswordForm.vue';

const message = ref('');
const error = ref('');
const devResetToken = ref('');

const submit = async (payload: { email: string }) => {
  message.value = '';
  error.value = '';
  devResetToken.value = '';

  const res = await fetch('http://127.0.0.1:8000/api/auth/forgot-password', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });

  if (!res.ok) {
    error.value = await res.text();
    return;
  }

  const data = await res.json();
  message.value =
    data.message || 'If an account exists, reset instructions were generated.';
  devResetToken.value = data.reset_token || '';
};
</script>

<template>
  <section class="auth-page">
    <div class="copy">
      <p class="eyebrow">Secure Recovery</p>
      <h1>Get back into your workspace.</h1>
      <p>
        The recovery flow generates a single-use reset token and prepares the
        platform for email-based recovery.
      </p>
    </div>

    <div>
      <ForgotPasswordForm :message="message" :error="error" @submit="submit" />

      <div v-if="devResetToken" class="dev-token">
        <p>Dev reset token:</p>
        <code>{{ devResetToken }}</code>
        <NuxtLink :to="`/reset-password?token=${devResetToken}`">
          Continue to reset password
        </NuxtLink>
      </div>
    </div>
  </section>
</template>

<style scoped>
.auth-page {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 440px;
  gap: 4rem;
  align-items: center;
  max-width: 1120px;
  margin: 0 auto;
}

.copy {
  color: white;
}

.eyebrow {
  color: #55d6be;
  text-transform: uppercase;
  letter-spacing: 0.12em;
}

h1 {
  max-width: 680px;
  font-size: clamp(2.4rem, 6vw, 4.5rem);
  line-height: 0.95;
  margin: 0 0 1rem;
}

.copy p {
  color: #a8bdd2;
  font-size: 1.1rem;
  line-height: 1.7;
}

.dev-token {
  margin-top: 1rem;
  padding: 1rem;
  border: 1px solid rgba(96, 165, 250, 0.35);
  border-radius: 1rem;
  color: #cde7ff;
  background: rgba(8, 19, 31, 0.8);
}

code {
  display: block;
  overflow-wrap: anywhere;
  margin-bottom: 0.75rem;
  color: #6ee7b7;
}

a {
  color: #7dd3fc;
}
</style>
