<!-- pages/admin/users.vue -->
<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import DashboardShell from '~/components/DashboardShell.vue';
import UserCreateForm from '~/components/UserCreateForm.vue';
import { useApi } from '~/composables/useApi';

definePageMeta({
  middleware: ['role'],
  allowedUserTypes: ['admin', 'super_admin'],
});

const api = useApi();

const users = ref<any[]>([]);
const loading = ref(true);
const saving = ref(false);
const error = ref('');
const query = ref('');
const selectedStatus = ref('all');
const selectedType = ref('all');

const userTypes = ['super_admin', 'admin', 'consultant', 'client'];

const filteredUsers = computed(() => {
  return users.value.filter((user) => {
    const haystack =
      `${user.name || ''} ${user.email || ''} ${user.user_type || ''}`.toLowerCase();
    const matchesQuery = haystack.includes(query.value.toLowerCase());

    const matchesType =
      selectedType.value === 'all' || user.user_type === selectedType.value;

    const isDisabled =
      user.status === 'disabled' ||
      user.disabled_at ||
      user.is_disabled === true;

    const matchesStatus =
      selectedStatus.value === 'all' ||
      (selectedStatus.value === 'active' && !isDisabled) ||
      (selectedStatus.value === 'disabled' && isDisabled);

    return matchesQuery && matchesType && matchesStatus;
  });
});

const stats = computed(() => {
  const total = users.value.length;
  const admins = users.value.filter((user) =>
    ['admin', 'super_admin'].includes(user.user_type),
  ).length;
  const clients = users.value.filter(
    (user) => user.user_type === 'client',
  ).length;
  const consultants = users.value.filter(
    (user) => user.user_type === 'consultant',
  ).length;

  return { total, admins, clients, consultants };
});

async function fetchUsers() {
  loading.value = true;
  error.value = '';

  try {
    users.value = await api.get('/api/admin/users');
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to load platform users.';
  } finally {
    loading.value = false;
  }
}

async function createUser(payload: any) {
  saving.value = true;
  error.value = '';

  try {
    await api.post('/api/admin/users', payload);
    await fetchUsers();
  } catch (err: any) {
    error.value = err?.data?.error || err?.message || 'Failed to create user.';
  } finally {
    saving.value = false;
  }
}

async function updateUserType(id: number, user_type: string) {
  error.value = '';

  try {
    await api.patch(`/api/admin/users/${id}/type`, { user_type });
    await fetchUsers();
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to update user role.';
  }
}

async function disableUser(id: number) {
  if (!confirm('Disable this user account?')) return;

  try {
    await api.patch(`/api/admin/users/${id}/disable`, {});
    await fetchUsers();
  } catch (err: any) {
    error.value = err?.data?.error || err?.message || 'Failed to disable user.';
  }
}

async function enableUser(id: number) {
  try {
    await api.patch(`/api/admin/users/${id}/enable`, {});
    await fetchUsers();
  } catch (err: any) {
    error.value = err?.data?.error || err?.message || 'Failed to enable user.';
  }
}

async function forcePasswordReset(id: number) {
  try {
    await api.post(`/api/admin/users/${id}/force-password-reset`, {});
    alert('Password reset requirement created.');
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to force password reset.';
  }
}

async function revokeSessions(id: number) {
  if (!confirm('Revoke all active sessions for this user?')) return;

  try {
    await api.delete(`/api/admin/users/${id}/sessions`);
    alert('User sessions revoked.');
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to revoke user sessions.';
  }
}

function userIsDisabled(user: any) {
  return user.status === 'disabled' || user.disabled_at || user.is_disabled;
}

function impersonateUser(user: any) {
  alert(
    `Impersonation target selected: ${user.email}. Next step is adding the backend impersonation token endpoint.`,
  );
}

onMounted(fetchUsers);
</script>

<template>
  <DashboardShell
    title="User Administration"
    subtitle="Create users, manage account access, inspect roles, and stabilize platform identity."
  >
    <section class="stats-grid">
      <div class="stat-card">
        <p>Total Users</p>
        <strong>{{ stats.total }}</strong>
      </div>

      <div class="stat-card">
        <p>Admins</p>
        <strong>{{ stats.admins }}</strong>
      </div>

      <div class="stat-card">
        <p>Consultants</p>
        <strong>{{ stats.consultants }}</strong>
      </div>

      <div class="stat-card">
        <p>Clients</p>
        <strong>{{ stats.clients }}</strong>
      </div>
    </section>

    <section class="panel">
      <div class="section-heading">
        <div>
          <p class="eyebrow">Create User</p>
          <h2>Invite or seed a platform account</h2>
        </div>
      </div>

      <UserCreateForm @submit="createUser" />

      <p v-if="saving" class="muted">Creating user...</p>
      <p v-if="error" class="error">{{ error }}</p>
    </section>

    <section class="panel">
      <div class="section-heading">
        <div>
          <p class="eyebrow">User Directory</p>
          <h2>Manage platform users</h2>
        </div>

        <button class="ghost-button" @click="fetchUsers">Refresh</button>
      </div>

      <div class="filters">
        <input
          v-model="query"
          class="form-input"
          placeholder="Search by name, email, or role..."
        />

        <select v-model="selectedType" class="form-input">
          <option value="all">All roles</option>
          <option v-for="type in userTypes" :key="type" :value="type">
            {{ type }}
          </option>
        </select>

        <select v-model="selectedStatus" class="form-input">
          <option value="all">All statuses</option>
          <option value="active">Active</option>
          <option value="disabled">Disabled</option>
        </select>
      </div>

      <div v-if="loading" class="empty-state">Loading users...</div>

      <div v-else-if="!filteredUsers.length" class="empty-state">
        No users match this view.
      </div>

      <div v-else class="table-wrap">
        <table>
          <thead>
            <tr>
              <th>User</th>
              <th>Role</th>
              <th>Status</th>
              <th>Created</th>
              <th>Admin Actions</th>
            </tr>
          </thead>

          <tbody>
            <tr v-for="user in filteredUsers" :key="user.id">
              <td>
                <strong>{{ user.name || 'Unnamed user' }}</strong>
                <span>{{ user.email }}</span>
              </td>

              <td>
                <select
                  class="compact-select"
                  :value="user.user_type"
                  @change="
                    updateUserType(
                      user.id,
                      ($event.target as HTMLSelectElement).value,
                    )
                  "
                >
                  <option v-for="type in userTypes" :key="type" :value="type">
                    {{ type }}
                  </option>
                </select>
              </td>

              <td>
                <span
                  class="badge"
                  :class="userIsDisabled(user) ? 'danger' : 'success'"
                >
                  {{ userIsDisabled(user) ? 'Disabled' : 'Active' }}
                </span>
              </td>

              <td>
                <span>{{ user.created_at || '—' }}</span>
              </td>

              <td>
                <div class="actions">
                  <NuxtLink :to="`/admin/users/${user.id}`" class="small-btn">
                    Details
                  </NuxtLink>

                  <button class="small-btn" @click="impersonateUser(user)">
                    Impersonate
                  </button>

                  <button
                    class="small-btn"
                    @click="forcePasswordReset(user.id)"
                  >
                    Reset
                  </button>

                  <button class="small-btn" @click="revokeSessions(user.id)">
                    Revoke Sessions
                  </button>

                  <button
                    v-if="userIsDisabled(user)"
                    class="small-btn success-btn"
                    @click="enableUser(user.id)"
                  >
                    Enable
                  </button>

                  <button
                    v-else
                    class="small-btn danger-btn"
                    @click="disableUser(user.id)"
                  >
                    Disable
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </section>
  </DashboardShell>
</template>

<style scoped>
.panel,
.stat-card {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 20px;
  background: linear-gradient(
    180deg,
    rgba(15, 23, 42, 0.96),
    rgba(2, 12, 23, 0.96)
  );
  color: #e5eefc;
  padding: 24px;
  margin-bottom: 22px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 18px;
  margin-bottom: 22px;
}

.stat-card p,
.muted {
  color: #a8bdd2;
  margin: 0 0 8px;
}

.stat-card strong {
  color: #6ee7b7;
  font-size: 2rem;
}

.section-heading {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  align-items: start;
  margin-bottom: 18px;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  margin: 0 0 8px;
}

h2 {
  margin: 0;
  color: #f8fafc;
}

.filters {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 180px 180px;
  gap: 12px;
  margin-bottom: 18px;
}

.form-input,
.compact-select {
  border: 1px solid rgba(45, 212, 191, 0.24);
  border-radius: 12px;
  background: rgba(2, 12, 23, 0.88);
  color: #f8fafc;
  padding: 11px 12px;
}

.ghost-button,
.small-btn {
  border: 1px solid rgba(45, 212, 191, 0.28);
  border-radius: 999px;
  background: rgba(8, 31, 42, 0.8);
  color: #dff7ff;
  cursor: pointer;
  font-weight: 800;
  padding: 8px 12px;
  text-decoration: none;
}

.table-wrap {
  overflow-x: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th {
  color: #67e8f9;
  font-size: 0.72rem;
  letter-spacing: 0.08em;
  text-align: left;
  text-transform: uppercase;
  padding: 12px;
}

td {
  border-top: 1px solid rgba(45, 212, 191, 0.16);
  padding: 14px 12px;
  vertical-align: top;
}

td strong,
td span {
  display: block;
}

td span {
  color: #a8bdd2;
  margin-top: 4px;
}

.badge {
  border-radius: 999px;
  display: inline-block;
  font-weight: 900;
  padding: 6px 10px;
}

.success {
  background: rgba(52, 211, 153, 0.14);
  color: #6ee7b7;
}

.danger {
  background: rgba(248, 113, 113, 0.14);
  color: #fca5a5;
}

.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.success-btn {
  border-color: rgba(52, 211, 153, 0.36);
}

.danger-btn {
  border-color: rgba(248, 113, 113, 0.36);
}

.error {
  color: #fca5a5;
}

.empty-state {
  border: 1px dashed rgba(45, 212, 191, 0.28);
  border-radius: 14px;
  color: #cbd5e1;
  padding: 18px;
}

@media (max-width: 860px) {
  .filters {
    grid-template-columns: 1fr;
  }
}
</style>
