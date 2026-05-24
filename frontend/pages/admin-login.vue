<script setup lang="ts">
import { navigateTo } from 'nuxt/app';
import { ref } from 'vue';
import LoginForm from '~/components/LoginForm.vue';
import { useAuth } from '~/composables/useAuth';
import { getPortalRoute } from '~/utils/authRedirect';

const error = ref('');
const resendMessage = ref('');
const showResendVerification = ref(false);
const resendLoading = ref(false);
const lastLoginEmail = ref('');

const { login: authLogin, resendVerification } = useAuth();

const resend = async () => {
  if (!lastLoginEmail.value) return;

  resendLoading.value = true;
  resendMessage.value = '';
  error.value = '';

  try {
    await resendVerification(lastLoginEmail.value);
    resendMessage.value =
      'If this account exists and is unverified, a verification email has been sent.';
  } catch (err: any) {
    error.value = err?.message || 'Unable to resend verification email.';
  } finally {
    resendLoading.value = false;
  }
};
const login = async (payload) => {
  error.value = '';

  try {
    const data = await authLogin(payload);
    await navigateTo(data.redirectTo);
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Unable to log in.';
  }
};
</script>

<template>
  <section class="auth-page">
    <div class="copy">
      <p class="eyebrow">Administrator Login</p>
      <h1>Access administrator tools.</h1>
      <p>
        Admins can manage orginzational level scope, assign users to
        organizations and give access rights to clients and consultants.
      </p>
    </div>

    <div>
      <LoginForm
        user-type="admin"
        submit-text="Open Admin Portal"
        @submit="login"
      >
        <template #extra>
          <NuxtLink class="helper-link" to="/forgot-password"
            >Forgot password?</NuxtLink
          >
        </template>
      </LoginForm>
      <p v-if="error" class="page-error">{{ error }}</p>
      <button
        v-if="showResendVerification"
        class="secondary-action"
        type="button"
        :disabled="resendLoading"
        @click="resend"
      >
        {{ resendLoading ? 'Sending...' : 'Resend verification email' }}
      </button>

      <p v-if="resendMessage" class="page-message">{{ resendMessage }}</p>
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
