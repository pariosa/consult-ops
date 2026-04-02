<!-- src/views/Auth/ClientLogin.vue -->
<template>
  <LoginForm
    submitText="Client Login"
    @submit="handleLogin"
  >
    <template #header>
      <h2>Client Portal</h2>
    </template>
  </LoginForm>
</template>

<script lang="ts" setup>
import LoginForm from '@/components/LoginForm.vue';
import { useAuthStore } from '@/stores/auth';

const auth = useAuthStore();

const handleLogin = async ({ email, password }: { email: string; password: string }) => {
  try {
    await auth.login(email, password); // backend can distinguish client
  } catch (err: any) {
    console.error(err);
  }
};
</script>