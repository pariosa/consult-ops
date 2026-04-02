<!-- src/views/Auth/ConsultantLogin.vue -->
<template>
  <LoginForm
    submitText="Consultant Login"
    @submit="handleLogin"
  >
    <template #header>
      <h2>Consultant Portal</h2>
    </template>
  </LoginForm>
</template>

<script lang="ts" setup>
import LoginForm from '@/components/LoginForm.vue';
import { useAuthStore } from '@/stores/auth';

const auth = useAuthStore();

const handleLogin = async ({ email, password }: { email: string; password: string }) => {
  try {
    // Pass type info to backend if needed
    await auth.login(email, password);
  } catch (err: any) {
    console.error(err);
  }
};
</script>