<script setup lang="ts">
import RegisterForm from '~/components/RegisterForm.vue';

const message = ref('');
const error = ref('');

const register = async (payload: {
  name: string;
  email: string;
  password: string;
  user_type: string;
}) => {
  message.value = '';
  error.value = '';

  const safePayload = {
    ...payload,
    // Do not allow public registration to create platform admins.
    user_type: payload.user_type === 'admin' ? 'consultant' : payload.user_type,
  };

  const res = await fetch('http://127.0.0.1:8000/api/auth/register', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(safePayload),
  });

  if (!res.ok) {
    error.value = await res.text();
    return;
  }

  await res.json();

  message.value =
    'Account created. Please verify your email before logging in.';
};
</script>

<template>
  <section class="auth-page">
    <div class="copy">
      <p class="eyebrow">Join Consult Ops</p>
      <h1>Create a workspace-ready account.</h1>
      <p>
        Register as a consultant or client user, then connect your account to
        organizations, projects, invoices, and operational workflows.
      </p>
    </div>

    <div>
      <RegisterForm
        user-type="consultant"
        submit-text="Create account"
        :message="message"
        :error="error"
        @submit="register"
      />

      <p class="helper">
        Already have an account?
        <NuxtLink to="/consultant-login">Login here</NuxtLink>
      </p>
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

.helper {
  color: #a8bdd2;
  margin-top: 1rem;
}

a {
  color: #7dd3fc;
}
</style>
