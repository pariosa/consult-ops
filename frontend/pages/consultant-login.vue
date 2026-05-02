<script setup lang="ts">
import { navigateTo } from 'nuxt/app';
import { ref } from 'vue';
import LoginForm from '~/components/LoginForm.vue';
import { getPortalRoute } from '~/utils/authRedirect';

const error = ref('');

const login = async (payload: {
  email: string;
  password: string;
  userType: string;
}) => {
  error.value = '';

  const res = await fetch('http://127.0.0.1:8000/api/auth/login', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      email: payload.email,
      password: payload.password,
    }),
  });

  if (!res.ok) {
    error.value = await res.text();
    return;
  }

  const data = await res.json();

  localStorage.setItem(
    'auth_user',
    JSON.stringify({
      ...data.user,
      token: data.token,
      portal: data.user.user_type || payload.userType,
    }),
  );

  await navigateTo(getPortalRoute(data.user.user_type || payload.userType));
};
</script>

<template>
  <section class="auth-page">
    <div class="copy">
      <p class="eyebrow">Consultant Workspace</p>
      <h1>Run your client operations from one place.</h1>
      <p>
        Track clients, projects, contracts, invoices, and payments from a single
        operational dashboard.
      </p>
    </div>

    <div>
      <LoginForm
        user-type="consultant"
        submit-text="Enter Workspace"
        @submit="login"
      >
        <template #extra>
          <NuxtLink class="helper-link" to="/forgot-password"
            >Forgot password?</NuxtLink
          >
        </template>
      </LoginForm>
      <p v-if="error" class="page-error">{{ error }}</p>
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

.helper-link {
  color: #7dd3fc;
  font-size: 0.9rem;
}

.page-error {
  color: #f87171;
  margin-top: 1rem;
}
</style>
