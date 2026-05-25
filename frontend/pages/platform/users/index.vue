<script setup lang="ts">
import { usePlatformAdmin } from '~/composables/usePlatformAdmin';

const { getUsers, createUser } = usePlatformAdmin();

const loading = ref(true);
const saving = ref(false);
const error = ref('');
const success = ref('');
const users = ref<any[]>([]);

const userForm = ref({
  email: '',
  name: '',
  user_type: 'admin',
  password: 'DemoPass123!',
});

async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    users.value = await getUsers();
  } catch (err: any) {
    error.value = err?.data?.error || err?.message || 'Failed to load users.';
  } finally {
    loading.value = false;
  }
}

async function submitUser() {
  saving.value = true;
  error.value = '';
  success.value = '';

  try {
    await createUser(userForm.value);
    success.value = 'User created.';
    userForm.value.email = '';
    userForm.value.name = '';
    await refresh();
  } catch (err: any) {
    error.value = err?.data?.error || err?.message || 'Failed to create user.';
  } finally {
    saving.value = false;
  }
}

onMounted(refresh);
</script>

<template>
  <DashboardShell
    title="Platform Users"
    subtitle="Create and manage platform users."
  >
    <section v-if="error" class="form-error">{{ error }}</section>
    <section v-if="success" class="success-state">{{ success }}</section>

    <section class="portal-section">
      <p class="eyebrow">Create User</p>

      <label>Email</label>
      <input v-model="userForm.email" class="form-input" />

      <label>Name</label>
      <input v-model="userForm.name" class="form-input" />

      <label>User Type</label>
      <select v-model="userForm.user_type" class="form-input">
        <option value="owner">Owner</option>
        <option value="admin">Admin</option>
        <option value="finance_admin">Finance Admin</option>
        <option value="operations_manager">Operations Manager</option>
        <option value="contractor">Contractor</option>
        <option value="client_viewer">Client Viewer</option>
        <option value="super_admin">Super Admin</option>
      </select>

      <label>Password</label>
      <input v-model="userForm.password" class="form-input" type="password" />

      <button
        class="form-button"
        :disabled="saving || !userForm.email"
        @click="submitUser"
      >
        Create User
      </button>
    </section>

    <section class="portal-section">
      <p class="eyebrow">Users</p>

      <div v-if="loading">Loading users...</div>

      <div v-else class="table-list">
        <NuxtLink
          v-for="user in users"
          :key="user.id"
          :to="`/platform/users/${user.id}`"
          class="table-row"
        >
          <span>{{ user.email }}</span>
          <span>{{ user.name }}</span>
          <span>{{ user.user_type }}</span>
        </NuxtLink>
      </div>
    </section>
  </DashboardShell>
</template>
