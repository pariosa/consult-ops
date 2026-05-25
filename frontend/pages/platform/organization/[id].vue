<script setup lang="ts">
import { usePlatformAdmin } from '~/composables/usePlatformAdmin';

const route = useRoute();
const { getUsers, getOrganizationMembers, assignUserToOrganization } =
  usePlatformAdmin();

const organizationId = computed(() => Number(route.params.id));

const users = ref<any[]>([]);
const members = ref<any[]>([]);
const loading = ref(true);
const saving = ref(false);
const error = ref('');
const success = ref('');

const assignmentForm = ref({
  user_id: null as number | null,
  role: 'admin',
});

async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    users.value = await getUsers();
    members.value = await getOrganizationMembers(organizationId.value);
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to load organization.';
  } finally {
    loading.value = false;
  }
}

async function submitAssignment() {
  if (!assignmentForm.value.user_id) return;

  saving.value = true;
  error.value = '';
  success.value = '';

  try {
    await assignUserToOrganization(organizationId.value, {
      user_id: assignmentForm.value.user_id,
      role: assignmentForm.value.role,
    });

    success.value = 'User assigned.';
    assignmentForm.value.user_id = null;
    await refresh();
  } catch (err: any) {
    error.value = err?.data?.error || err?.message || 'Failed to assign user.';
  } finally {
    saving.value = false;
  }
}

onMounted(refresh);
</script>

<template>
  <DashboardShell
    title="Organization Detail"
    subtitle="Manage organization membership."
  >
    <section v-if="error" class="form-error">{{ error }}</section>
    <section v-if="success" class="success-state">{{ success }}</section>

    <section class="portal-section">
      <p class="eyebrow">Assign User</p>

      <label>User</label>
      <select v-model.number="assignmentForm.user_id" class="form-input">
        <option :value="null">Select user</option>
        <option v-for="user in users" :key="user.id" :value="user.id">
          {{ user.email }} — {{ user.user_type }}
        </option>
      </select>

      <label>Role</label>
      <select v-model="assignmentForm.role" class="form-input">
        <option value="owner">Owner</option>
        <option value="admin">Admin</option>
        <option value="finance_admin">Finance Admin</option>
        <option value="operations_manager">Operations Manager</option>
        <option value="contractor">Contractor</option>
        <option value="client_viewer">Client Viewer</option>
        <option value="member">Member</option>
      </select>

      <button
        class="form-button"
        :disabled="saving || !assignmentForm.user_id"
        @click="submitAssignment"
      >
        Assign User
      </button>
    </section>

    <section class="portal-section">
      <p class="eyebrow">Members</p>

      <div v-if="loading">Loading members...</div>
      <div v-else-if="!members.length" class="empty-state">
        No members found.
      </div>

      <div v-else class="table-list">
        <div v-for="member in members" :key="member.id" class="table-row">
          <span>{{ member.email }}</span>
          <span>{{ member.name }}</span>
          <span>{{ member.user_type }}</span>
          <span>{{ member.role }}</span>
          <span>{{ member.status }}</span>
        </div>
      </div>
    </section>
  </DashboardShell>
</template>
