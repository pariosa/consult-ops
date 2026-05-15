<script setup lang="ts">
import InvitationTable from '~/components/Organization/InvitationTable.vue';
import { useApi } from '~/composables/useApi';
import { useOrganizationMembers } from '~/composables/useOrganizationMembers';

const api = useApi();
const { getInvitations, inviteMember } = useOrganizationMembers();

const loading = ref(true);
const saving = ref(false);
const error = ref('');
const success = ref('');
const organization = ref<any>(null);
const invitations = ref<any[]>([]);
const latestInvite = ref<any>(null);

const form = ref({
  email: '',
  role: 'contractor',
});

async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    organization.value = await api.get('/api/me/organization');
    invitations.value = await getInvitations(organization.value.id);
  } catch (err: any) {
    error.value = err?.message || 'Failed to load invitations.';
  } finally {
    loading.value = false;
  }
}

async function submitInvite() {
  if (!organization.value?.id) return;

  saving.value = true;
  error.value = '';
  success.value = '';
  latestInvite.value = null;

  try {
    latestInvite.value = await inviteMember(organization.value.id, form.value);
    success.value = 'Invitation created.';
    form.value.email = '';
    await refresh();
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to invite member.';
  } finally {
    saving.value = false;
  }
}

onMounted(refresh);
</script>

<template>
  <DashboardShell
    title="Organization Invitations"
    subtitle="Invite contractors, finance admins, operators, and clients into your workspace."
  >
    <section v-if="loading" class="portal-section">
      Loading invitations...
    </section>

    <section v-else-if="error" class="form-error">{{ error }}</section>

    <template v-else>
      <section class="portal-section">
        <p class="eyebrow">Invite User</p>
        <h2>Send Organization Invitation</h2>

        <div class="form-grid">
          <label>Email</label>
          <input v-model="form.email" class="form-input" type="email" />

          <label>Role</label>
          <select v-model="form.role" class="form-input">
            <option value="admin">Admin</option>
            <option value="finance_admin">Finance Admin</option>
            <option value="operations_manager">Operations Manager</option>
            <option value="contractor">Contractor</option>
            <option value="client_viewer">Client Viewer</option>
            <option value="member">Member</option>
          </select>

          <button
            class="form-button"
            :disabled="saving || !form.email"
            @click="submitInvite"
          >
            {{ saving ? 'Sending...' : 'Create Invite' }}
          </button>
        </div>

        <div v-if="success" class="success-state">{{ success }}</div>

        <div v-if="latestInvite?.invite_url" class="invite-preview">
          <p class="eyebrow">Dev Email Preview</p>
          <p><strong>To:</strong> {{ latestInvite.email_preview.to }}</p>
          <p>
            <strong>Subject:</strong> {{ latestInvite.email_preview.subject }}
          </p>
          <p>{{ latestInvite.email_preview.body }}</p>
          <code>{{ latestInvite.invite_url }}</code>
        </div>
      </section>

      <section class="portal-section">
        <p class="eyebrow">Pending & Past Invites</p>
        <h2>Invitations</h2>
        <InvitationTable :invitations="invitations" />
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

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 800;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

h2 {
  color: #f8fafc;
}

.form-grid {
  display: grid;
  gap: 12px;
}

.form-input {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 12px;
  background: rgba(2, 12, 23, 0.95);
  color: #f8fafc;
  padding: 12px 14px;
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

.form-error {
  border: 1px solid rgba(248, 113, 113, 0.38);
  border-radius: 14px;
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
  padding: 16px;
}

.success-state {
  border: 1px solid rgba(52, 211, 153, 0.3);
  border-radius: 14px;
  color: #6ee7b7;
  margin-top: 16px;
  padding: 14px;
}

.invite-preview {
  background: rgba(8, 31, 42, 0.86);
  border: 1px solid rgba(45, 212, 191, 0.18);
  border-radius: 14px;
  margin-top: 16px;
  padding: 16px;
}

code {
  display: block;
  color: #93c5fd;
  overflow-wrap: anywhere;
}
</style>
