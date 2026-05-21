<script setup lang="ts">
import { useRoute } from 'nuxt/app';
import { computed, ref } from 'vue';
import ResetPasswordForm from '~/components/ResetPasswordForm.vue';
import { useApi } from '~/composables/useApi';

const route = useRoute();

const message = ref('');
const error = ref('');

const token = computed(() => {
  const value = route.query.token;
  return typeof value === 'string' ? value : '';
});
const { post } = useApi();
const submit = async (payload: { password: string }) => {
  message.value = '';
  error.value = '';

  if (!token.value) {
    error.value = 'Missing reset token.';
    return;
  }

  const res = await post('http://127.0.0.1:8000/api/auth/reset-password', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      token: token.value,
      password: payload.password,
    }),
  });

  if (!res.ok) {
    error.value = await res.text();
    return;
  }

  message.value = await res.text();
};
</script>

<template>
  <section class="auth-page">
    <div class="copy">
      <p class="eyebrow">Secure Recovery</p>
      <h1>Create a new account password.</h1>
      <p>Reset links are token-based, time-limited, and single-use.</p>
    </div>

    <ResetPasswordForm :message="message" :error="error" @submit="submit" />
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
</style>
