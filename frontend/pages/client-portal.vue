<!-- frontend/pages/client-portal.vue -->
<script setup lang="ts">
import { useApi } from '~/composables/useApi';

definePageMeta({
  middleware: ['role'],
  roles: ['client'],
});

const { apiFetch } = useApi();

const { data: summary, pending } = await useAsyncData(
  'client-portal-summary',
  () => {
    return apiFetch('/api/client-portal/summary');
  },
);
</script>

<template>
  <DashboardShell
    title="Client Portal"
    subtitle="View your projects, invoices, and contracts"
  >
    <p v-if="pending">Loading portal...</p>

    <section v-else class="dashboard-grid">
      <KPITile label="Active Projects" :value="summary?.active_projects ?? 0" />
      <KPITile label="Open Invoices" :value="summary?.open_invoices ?? 0" />
      <KPITile label="Contracts" :value="summary?.contracts ?? 0" />
    </section>
  </DashboardShell>
</template>
