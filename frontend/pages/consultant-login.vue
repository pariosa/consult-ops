<script setup lang="ts">
import { ref } from 'vue';
import LoginForm from '~/components/LoginForm.vue';
import { useAuth } from '~/composables/useAuth';

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

const login = async (payload: {
  email: string;
  password: string;
  userType: string;
  remember_me: boolean;
}) => {
  error.value = '';
  resendMessage.value = '';
  showResendVerification.value = false;
  lastLoginEmail.value = payload.email;

  try {
    const data = await authLogin(payload);
    await navigateTo(data.redirectTo);
  } catch (err: any) {
    error.value = err?.message || 'Unable to log in.';

    if (err?.code === 'EMAIL_VERIFICATION_REQUIRED') {
      showResendVerification.value = true;
    }
  }
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
.secondary-action {
  margin-top: 0.75rem;
  border: 1px solid rgba(125, 211, 252, 0.45);
  border-radius: 0.75rem;
  padding: 0.7rem 0.9rem;
  background: rgba(14, 165, 233, 0.08);
  color: #7dd3fc;
  font-weight: 700;
  cursor: pointer;
}

.page-message {
  color: #6ee7b7;
  margin-top: 0.75rem;
}
</style>
