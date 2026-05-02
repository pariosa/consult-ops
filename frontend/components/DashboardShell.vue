<!-- components/DashboardShell.vue -->
<template>
  <section class="dashboard-shell">
    <aside class="sidebar">
      <NuxtLink to="/admin">Admin Home</NuxtLink>
      <NuxtLink to="/admin/users">Users</NuxtLink>
      <NuxtLink to="/organization">Organization</NuxtLink>
      <NuxtLink to="/organization/members">Members</NuxtLink>
      <NuxtLink to="/organization/clients">Clients</NuxtLink>
      <NuxtLink to="/organization/projects">Projects</NuxtLink>
      <NuxtLink to="/profile">Profile</NuxtLink>
    </aside>

    <main class="content">
      <slot />
    </main>
  </section>
</template>

<style scoped>
.dashboard-shell {
  display: grid;
  grid-template-columns: 260px 1fr;
  gap: 2rem;
  min-height: calc(100vh - 96px);
}

.sidebar {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  padding: 1.25rem;
  border: 1px solid rgba(80, 210, 170, 0.25);
  border-radius: 1.25rem;
  background: rgba(8, 19, 31, 0.82);
}

.sidebar a {
  color: #cde7ff;
  text-decoration: none;
}

.sidebar a:hover {
  color: #6ee7b7;
}

.content {
  color: white;
}
</style>
