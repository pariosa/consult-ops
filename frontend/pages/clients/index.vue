<!-- frontend/pages/organization/clients.vue -->
<script setup lang="ts">
import { useApi } from '~/composables/useApi';
import { useOrganizationStore } from '~/stores/organization';

definePageMeta({
  middleware: ['role'],
  roles: ['admin', 'consultant'],
});

const { apiFetch } = useApi();
const orgStore = useOrganizationStore();

const {
  data: clients,
  pending,
  error,
} = await useAsyncData('organization-clients', async () => {
  await orgStore.fetchCurrentOrganization();

  if (!orgStore.organizationId) return [];

  return await apiFetch(
    `/api/organizations/${orgStore.organizationId}/clients`,
  );
});
</script>

<template>
  <DashboardShell
    title="Organization Clients"
    subtitle="Manage client accounts"
  >
    <p v-if="pending">Loading clients...</p>
    <p v-else-if="error">Could not load clients.</p>

    <section v-else class="card-grid">
      <Card v-for="client in clients" :key="client.id">
        <h3>{{ client.name }}</h3>
        <p>{{ client.email || 'No email set' }}</p>
      </Card>
    </section>
  </DashboardShell>
</template>
