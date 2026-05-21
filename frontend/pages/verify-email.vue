<script setup lang="ts">
const route = useRoute();
const config = useRuntimeConfig();

const status = ref<'loading' | 'success' | 'error'>('loading');
const message = ref('Verifying your email...');

onMounted(async () => {
  const token = route.query.token;

  if (!token || typeof token !== 'string') {
    status.value = 'error';
    message.value = 'Verification token is missing.';
    return;
  }

  try {
    const res = await fetch(`${config.public.apiBase}/api/auth/verify-email`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ token }),
    });

    if (!res.ok) {
      status.value = 'error';
      message.value = await res.text();
      return;
    }

    status.value = 'success';
    message.value = 'Your email has been verified. You can now log in.';
  } catch {
    status.value = 'error';
    message.value = 'Unable to verify your email.';
  }
});
</script>

<template>
  <section class="verify-page">
    <div class="card">
      <p class="eyebrow">Email Verification</p>

      <h1 v-if="status === 'loading'">Verifying...</h1>
      <h1 v-else-if="status === 'success'">Email verified</h1>
      <h1 v-else>Verification failed</h1>

      <p :class="status">{{ message }}</p>

      <NuxtLink to="/consultant-login" class="button">
        Continue to login
      </NuxtLink>
    </div>
  </section>
</template>

<style scoped>
.verify-page {
  min-height: 70vh;
  display: grid;
  place-items: center;
  color: white;
}

.card {
  width: min(100%, 460px);
  padding: 2rem;
  border: 1px solid rgba(80, 210, 170, 0.35);
  border-radius: 1.25rem;
  background: #08131f;
}

.eyebrow {
  color: #55d6be;
  text-transform: uppercase;
  letter-spacing: 0.12em;
}

.success {
  color: #6ee7b7;
}

.error {
  color: #f87171;
}

.loading {
  color: #bfdbfe;
}

.button {
  display: inline-block;
  margin-top: 1rem;
  color: #7dd3fc;
}
</style>
