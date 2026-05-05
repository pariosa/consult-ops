<!-- components/DashboardShell.vue -->
<script setup lang="ts">
import { computed } from 'vue';
import { useAuth } from '~/composables/useAuth';
import { navItems } from '~/utils/navItems';

defineProps<{
  title: string;
  subtitle?: string;
}>();

const { authUser } = useAuth();

const visibleNavItems = computed(() => {
  const userType = authUser.value?.user_type || authUser.value?.role;

  return navItems.filter((item) => item.roles.includes(userType));
});
</script>

<template>
  <div class="dashboard-layout">
    <aside class="sidebar">
      <NuxtLink v-for="item in visibleNavItems" :key="item.to" :to="item.to">
        {{ item.label }}
      </NuxtLink>
    </aside>

    <main class="dashboard-main">
      <header class="dashboard-header">
        <h1>{{ title }}</h1>
        <p v-if="subtitle">{{ subtitle }}</p>
      </header>

      <slot />
    </main>
  </div>
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
.dashboard-header h1 {
  color: white;
}

.dashboard-header p {
  color: #9fb3c8;
}
</style>
