<!-- frontend/pages/organization/projects.vue -->
<script setup lang="ts">
import { useAsyncData } from 'nuxt/app';
import { definePageMeta } from 'nuxt/dist/pages/runtime';
import { useApi } from '~/composables/useApi';
import { useOrganizationStore } from '~/stores/organization';

definePageMeta({
  middleware: ['role'],
  roles: ['admin', 'consultant'],
});

const { apiFetch } = useApi();
const orgStore = useOrganizationStore();

const {
  data: projects,
  pending,
  error,
} = await useAsyncData('organization-projects', async () => {
  await orgStore.fetchCurrentOrganization();

  if (!orgStore.organizationId) return [];

  return await apiFetch(
    `/api/organizations/${orgStore.organizationId}/projects`,
  );
});
</script>

<template>
  <DashboardShell
    title="Organization Projects"
    subtitle="Track active client work"
  >
    <p v-if="pending">Loading projects...</p>
    <p v-else-if="error">Could not load projects.</p>

    <section v-else class="card-grid">
      <ProjectCard
        v-for="project in projects"
        :key="project.id"
        :project="project"
      />
    </section>
  </DashboardShell>
</template>
