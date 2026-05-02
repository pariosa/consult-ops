<!-- frontend/pages/project-portal.vue -->
<script setup lang="ts">
import { useAsyncData } from 'nuxt/app';
import { definePageMeta } from 'nuxt/dist/pages/runtime';
import { useApi } from '~/composables/useApi';

definePageMeta({
  middleware: ['role'],
  roles: ['admin', 'consultant'],
});

const { apiFetch } = useApi();

const { data: summary, pending } = await useAsyncData(
  'project-portal-summary',
  () => {
    return apiFetch('/api/project-portal/summary');
  },
);
</script>

<template>
  <DashboardShell
    title="Project Portal"
    subtitle="Manage consulting delivery work"
  >
    <p v-if="pending">Loading projects...</p>

    <section v-else class="dashboard-grid">
      <KPITile
        label="Assigned Projects"
        :value="summary?.assigned_projects ?? 0"
      />
      <KPITile label="Active Clients" :value="summary?.active_clients ?? 0" />
      <KPITile
        label="Pending Invoices"
        :value="summary?.pending_invoices ?? 0"
      />
    </section>
  </DashboardShell>
</template>
