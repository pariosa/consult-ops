<!-- frontend/pages/organization/members.vue -->
<script setup lang="ts">
import MemberTable from '~/components/Organization/MemberTable.vue';
import { useApi } from '~/composables/useApi';
import { useOrganizationMembers } from '~/composables/useOrganizationMembers';

const api = useApi();
const { getMembers } = useOrganizationMembers();

const loading = ref(true);
const error = ref('');
const organization = ref<any>(null);
const members = ref<any[]>([]);

async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    organization.value = await api.get('/api/me/organization');
    members.value = await getMembers(organization.value.id);
  } catch (err: any) {
    error.value = err?.message || 'Failed to load organization members.';
  } finally {
    loading.value = false;
  }
}

onMounted(refresh);
</script>

<template>
  <DashboardShell
    title="Organization Members"
    subtitle="Manage the people who can operate inside this workspace."
  >
    <section v-if="loading" class="portal-section">Loading members...</section>

    <section v-else-if="error" class="form-error">{{ error }}</section>

    <section v-else class="portal-section">
      <div class="section-header">
        <div>
          <p class="eyebrow">Team Access</p>
          <h2>{{ organization?.name }}</h2>
        </div>

        <NuxtLink class="form-button secondary" to="/organization/invitations">
          Invite Members
        </NuxtLink>
      </div>

      <MemberTable :members="members" />
    </section>
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
}

.section-header {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 20px;
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

.form-button.secondary {
  background: rgba(15, 23, 42, 0.95);
  border: 1px solid rgba(45, 212, 191, 0.32);
  border-radius: 12px;
  color: #e5eefc;
  font-weight: 800;
  padding: 12px 16px;
  text-decoration: none;
}

.form-error {
  border: 1px solid rgba(248, 113, 113, 0.38);
  border-radius: 14px;
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
  padding: 16px;
}
</style>
