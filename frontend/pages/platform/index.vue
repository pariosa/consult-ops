<script setup lang="ts">
import { usePlatformAdmin } from '~/composables/usePlatformAdmin';
import { usePermissions } from '~/composables/usePermissions';

const {
  getOrganizations,
  createOrganization,
  getUsers,
  createUser,
  getOrganizationMembers,
  assignUserToOrganization,
} = usePlatformAdmin();

const { role } = usePermissions();

const loading = ref(true);
const saving = ref(false);
const error = ref('');
const success = ref('');

const organizations = ref<any[]>([]);
const users = ref<any[]>([]);
const selectedOrganizationId = ref<number | null>(null);
const members = ref<any[]>([]);

const organizationForm = ref({
  name: '',
});

const userForm = ref({
  email: '',
  name: '',
  user_type: 'admin',
  password: 'DemoPass123!',
});

const assignmentForm = ref({
  user_id: null as number | null,
  role: 'admin',
});

const isSuperAdmin = computed(() => role.value === 'super_admin');

async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    organizations.value = await getOrganizations();
    users.value = await getUsers();

    if (!selectedOrganizationId.value && organizations.value.length) {
      selectedOrganizationId.value = organizations.value[0].id;
    }

    await refreshMembers();
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to load platform admin.';
  } finally {
    loading.value = false;
  }
}

async function refreshMembers() {
  if (!selectedOrganizationId.value) {
    members.value = [];
    return;
  }

  members.value = await getOrganizationMembers(selectedOrganizationId.value);
}

async function submitOrganization() {
  saving.value = true;
  error.value = '';
  success.value = '';

  try {
    await createOrganization(organizationForm.value);
    success.value = 'Organization created.';
    organizationForm.value.name = '';
    await refresh();
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to create organization.';
  } finally {
    saving.value = false;
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

async function submitAssignment() {
  if (!selectedOrganizationId.value || !assignmentForm.value.user_id) return;

  saving.value = true;
  error.value = '';
  success.value = '';

  try {
    await assignUserToOrganization(selectedOrganizationId.value, {
      user_id: assignmentForm.value.user_id,
      role: assignmentForm.value.role,
    });
    success.value = 'User assigned to organization.';
    await refreshMembers();
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
    title="Platform Admin"
    subtitle="Super-admin tools for organizations, users, and membership recovery."
  >
    <section v-if="!isSuperAdmin" class="form-error">
      Platform admin access required.
    </section>

    <section v-else-if="loading" class="portal-section">
      Loading platform admin...
    </section>

    <template v-else>
      <section v-if="error" class="form-error">{{ error }}</section>
      <section v-if="success" class="success-state">{{ success }}</section>

      <section class="platform-grid">
        <div class="portal-section">
          <p class="eyebrow">Organizations</p>
          <h2>Create Organization</h2>
          <label for="platform-org-name">Organization Name</label>
          <input
            id="platform-org-name"
            v-model="organizationForm.name"
            class="form-input"
          />
          <button
            class="form-button"
            :disabled="saving || !organizationForm.name"
            @click="submitOrganization"
          >
            Create Organization
          </button>
        </div>

        <div class="portal-section">
          <p class="eyebrow">Users</p>
          <h2>Create User</h2>

          <label for="platform-user-email">Email</label>
          <input
            id="platform-user-email"
            v-model="userForm.email"
            class="form-input"
          />

          <label for="platform-user-name">Name</label>
          <input
            id="platform-user-name"
            v-model="userForm.name"
            class="form-input"
          />

          <label for="platform-user-type">User Type</label>
          <select
            id="platform-user-type"
            v-model="userForm.user_type"
            class="form-input"
          >
            <option value="owner">Owner</option>
            <option value="admin">Admin</option>
            <option value="finance_admin">Finance Admin</option>
            <option value="operations_manager">Operations Manager</option>
            <option value="contractor">Contractor</option>
            <option value="client_viewer">Client Viewer</option>
            <option value="super_admin">Super Admin</option>
          </select>

          <label for="platform-user-password">Password</label>
          <input
            id="platform-user-password"
            v-model="userForm.password"
            class="form-input"
            type="password"
          />

          <button
            class="form-button"
            :disabled="saving || !userForm.email"
            @click="submitUser"
          >
            Create User
          </button>
        </div>
      </section>

      <section class="portal-section">
        <p class="eyebrow">Organization Membership</p>
        <h2>Assign User To Organization</h2>

        <div class="form-grid">
          v
          <label for="platform-assignment-organization">Organization</label>
          <select
            id="platform-assignment-organization"
            v-model.number="selectedOrganizationId"
            class="form-input"
          >
            <option v-for="org in organizations" :key="org.id" :value="org.id">
              {{ org.name }}
            </option>
          </select>

          <label for="platform-assignment-user">User</label>
          <select
            id="platform-assignment-user"
            v-model.number="assignmentForm.user_id"
            class="form-input"
          >
            <option :value="null">Select user</option>
            <option v-for="user in users" :key="user.id" :value="user.id">
              {{ user.email }} — {{ user.user_type }}
            </option>
          </select>

          <label for="platform-assignment-role">Role</label>
          <select
            id="platform-assignment-role"
            v-model="assignmentForm.role"
            class="form-input"
          >
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
        </div>
      </section>

      <section class="portal-section">
        <p class="eyebrow">Current Members</p>
        <h2>Selected Organization Members</h2>

        <div v-if="!members.length" class="empty-state">No members found.</div>

        <div v-else class="table-list">
          <div class="table-row table-row--header">
            <span>Email</span>
            <span>Name</span>
            <span>User Type</span>
            <span>Org Role</span>
            <span>Status</span>
          </div>

          <div v-for="member in members" :key="member.id" class="table-row">
            <span>{{ member.email }}</span>
            <span>{{ member.name }}</span>
            <span>{{ member.user_type }}</span>
            <span>{{ member.role }}</span>
            <span>{{ member.status }}</span>
          </div>
        </div>
      </section>
    </template>
  </DashboardShell>
</template>

<style scoped>
.portal-section {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 18px;
  background: linear-gradient(
    180deg,
    rgba(15, 23, 42, 0.96),
    rgba(2, 12, 23, 0.96)
  );
  color: #e5eefc;
  padding: 24px;
  margin-bottom: 24px;
}

.platform-grid {
  display: grid;
  gap: 18px;
}

.form-grid {
  display: grid;
  gap: 12px;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

h2 {
  color: #f8fafc;
}

label {
  color: #cbd5e1;
}

.form-input {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 12px;
  background: rgba(2, 12, 23, 0.95);
  color: #f8fafc;
  padding: 12px 14px;
  width: 100%;
}

.form-button {
  border: 0;
  border-radius: 12px;
  background: linear-gradient(90deg, #60a5fa, #34d399);
  color: #04111f;
  cursor: pointer;
  font-weight: 800;
  padding: 12px 16px;
}

.form-error,
.success-state,
.empty-state {
  border-radius: 14px;
  margin-bottom: 16px;
  padding: 16px;
}

.form-error {
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
}

.success-state {
  border: 1px solid rgba(52, 211, 153, 0.28);
  color: #6ee7b7;
}

.empty-state {
  border: 1px dashed rgba(45, 212, 191, 0.28);
  color: #cbd5e1;
}

.table-list {
  display: grid;
  gap: 10px;
}

.table-row {
  display: grid;
  grid-template-columns: 1.4fr 1fr 0.8fr 0.8fr 0.7fr;
  gap: 12px;
  border: 1px solid rgba(45, 212, 191, 0.18);
  border-radius: 14px;
  background: rgba(8, 31, 42, 0.86);
  color: #cbd5e1;
  padding: 14px;
}

.table-row--header {
  background: rgba(2, 12, 23, 0.92);
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.1em;
  text-transform: uppercase;
}

@media (min-width: 900px) {
  .platform-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
</style>
