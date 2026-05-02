<!-- frontend/pages/organization/index.vue -->
<script setup lang="ts">
import { useAsyncData } from 'nuxt/app';
import { definePageMeta } from 'nuxt/dist/pages/runtime';
import { useOrganizationStore } from '~/stores/organization';

definePageMeta({
  middleware: ['role'],
  roles: ['admin', 'consultant', 'client'],
});

const orgStore = useOrganizationStore();

await useAsyncData('organization-overview', async () => {
  await orgStore.fetchCurrentOrganization();
  await orgStore.fetchMembers();
  return true;
});
</script>

<template>
  <DashboardShell title="Organization" subtitle="Manage your company workspace">
    <section v-if="orgStore.loading">Loading organization...</section>

    <section v-else-if="orgStore.error" class="error">
      {{ orgStore.error }}
    </section>

    <section v-else class="dashboard-grid">
      <KPITile
        label="Organization"
        :value="orgStore.organization?.name || '—'"
      />
      <KPITile label="Members" :value="orgStore.memberCount" />
      <KPITile label="Projects" value="Coming soon" />
      <KPITile label="Clients" value="Coming soon" />
    </section>
  </DashboardShell>
</template>
