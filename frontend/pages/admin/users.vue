<!-- pages/admin/users.vue -->
<script setup lang="ts">
import { onMounted } from 'vue';
import DashboardShell from '~/components/DashboardShell.vue';
import UserCreateForm from '~/components/UserCreateForm.vue';
import UserTable from '~/components/UserTable.vue';

definePageMeta({
  middleware: ['role'],
  allowedUserTypes: ['admin'],
});

const users = ref<any[]>([]);
const error = ref('');

const fetchUsers = async () => {
  const res = await fetch('http://127.0.0.1:8000/api/admin/users');
  users.value = await res.json();
};

const createUser = async (payload: any) => {
  error.value = '';

  const res = await fetch('http://127.0.0.1:8000/api/admin/users', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });

  if (!res.ok) {
    error.value = await res.text();
    return;
  }

  await fetchUsers();
};

const updateUserType = async (payload: { id: number; user_type: string }) => {
  const res = await fetch(
    `http://127.0.0.1:8000/api/admin/users/${payload.id}/type`,
    {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ user_type: payload.user_type }),
    },
  );

  if (res.ok) await fetchUsers();
};

onMounted(fetchUsers);
</script>

<template>
  <DashboardShell>
    <h1>User Administration</h1>
    <p class="intro">Create users and manage platform-level account types.</p>

    <UserCreateForm @submit="createUser" />
    <p v-if="error" class="error">{{ error }}</p>

    <UserTable :users="users" @update-user-type="updateUserType" />
  </DashboardShell>
</template>

<style scoped>
.intro {
  color: #a8bdd2;
}
.error {
  color: #f87171;
}
</style>
