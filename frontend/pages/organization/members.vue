<!-- frontend/pages/organization/members.vue -->
<script setup lang="ts">
import { useAsyncData } from 'nuxt/app';
import { definePageMeta } from 'nuxt/dist/pages/runtime';
import { useOrganizationStore } from '~/stores/organization';

definePageMeta({
  middleware: ['role'],
  roles: ['admin', 'consultant'],
});

const orgStore = useOrganizationStore();

await useAsyncData('organization-members', async () => {
  await orgStore.fetchCurrentOrganization();
  await orgStore.fetchMembers();
  return true;
});
</script>

<template>
  <DashboardShell
    title="Organization Members"
    subtitle="View and manage workspace members"
  >
    <table class="data-table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Email</th>
          <th>Org Role</th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="member in orgStore.members" :key="member.id">
          <td>{{ member.name || '—' }}</td>
          <td>{{ member.email }}</td>
          <td>{{ member.role }}</td>
        </tr>
      </tbody>
    </table>
  </DashboardShell>
</template>
