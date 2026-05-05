<!-- pages/admin/index.vue -->
<script setup lang="ts">
import DashboardShell from '~/components/DashboardShell.vue';

definePageMeta({
  middleware: ['role'],
  allowedUserTypes: ['admin'],
});
</script>

<template>
  <DashboardShell
    title="Admin Panel"
    subtitle="Manage users, organizations, roles, and platform settings."
  >
    <div class="grid">
      <NuxtLink to="/admin/users" class="card">User Management</NuxtLink>
      <NuxtLink to="/organization" class="card">Organizations</NuxtLink>
      <NuxtLink to="/organization/projects" class="card">Projects</NuxtLink>
      <NuxtLink to="/organization/clients" class="card">Clients</NuxtLink>
    </div>
  </DashboardShell>
</template>

<style scoped>
p {
  color: #a8bdd2;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 1rem;
  margin-top: 2rem;
}

.card {
  padding: 1.5rem;
  border-radius: 1.25rem;
  color: white;
  text-decoration: none;
  background: linear-gradient(
    135deg,
    rgba(37, 99, 235, 0.25),
    rgba(16, 185, 129, 0.18)
  );
  border: 1px solid rgba(80, 210, 170, 0.25);
}
</style>
