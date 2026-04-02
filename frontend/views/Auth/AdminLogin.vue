<!-- src/views/Auth/AdminLogin.vue -->
<template>
  <LoginForm
    submitText="Admin Login"
    @submit="handleLogin"
  >
    <template #header>
      <h2>Admin Portal</h2>
    </template>
    <template #extra>
      <router-link to="/forgot-password">Forgot password?</router-link>
    </template>
  </LoginForm>
</template>

<script lang="ts" setup>
import LoginForm from '@/components/LoginForm.vue';
import { useAuthStore } from '@/stores/auth';

const auth = useAuthStore();

const handleLogin = async ({ email, password }: { email: string; password: string }) => {
  try {
    await auth.login(email, password); // backend knows this is admin login
  } catch (err: any) {
    console.error(err);
  }
};
</script>