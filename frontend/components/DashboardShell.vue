<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { useAuth } from '~/composables/useAuth';
import { useNotifications } from '~/composables/useNotifications';
import { navItems } from '~/utils/navItems';

defineProps<{
  title: string;
  subtitle?: string;
}>();

const { authUser } = useAuth();
const { unreadCount, refreshNotifications } = useNotifications();

const userType = computed(
  () => authUser.value?.user_type || authUser.value?.role || 'member',
);

const visibleNavItems = computed(() =>
  navItems.filter((item) => item.roles.includes(userType.value)),
);

onMounted(refreshNotifications);
</script>

<template>
  <div class="dashboard-layout">
    <aside class="sidebar">
      <div class="sidebar-brand">
        <NuxtLink to="/organization" class="brand-link">Consult Ops</NuxtLink>
        <span class="role-pill">{{ userType }}</span>
      </div>

      <NuxtLink
        v-for="item in visibleNavItems"
        :key="item.to"
        :to="item.to"
        class="nav-link"
      >
        {{ item.label }}
      </NuxtLink>
    </aside>

    <main class="dashboard-main">
      <header class="dashboard-header">
        <div>
          <h1>{{ title }}</h1>
          <p v-if="subtitle">{{ subtitle }}</p>
        </div>

        <NuxtLink
          to="/notifications"
          class="notification-bell"
          aria-label="Notifications"
        >
          <span class="bell-icon">🔔</span>
          <span v-if="unreadCount > 0" class="notification-badge">
            {{ unreadCount }}
          </span>
        </NuxtLink>
      </header>

      <slot />
    </main>
  </div>
</template>

<style scoped>
.dashboard-layout {
  display: grid;
  grid-template-columns: 240px minmax(0, 1fr);
  gap: 24px;
  min-height: calc(100vh - 96px);
  padding: 24px;
}

.sidebar {
  align-self: start;
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 18px;
  border: 1px solid rgba(45, 212, 191, 0.25);
  border-radius: 18px;
  background: rgba(8, 19, 31, 0.86);
}

.sidebar-brand {
  border-bottom: 1px solid rgba(45, 212, 191, 0.18);
  margin-bottom: 10px;
  padding-bottom: 14px;
}

.brand-link {
  color: #f8fafc;
  display: block;
  font-weight: 900;
  margin-bottom: 8px;
  text-decoration: none;
}

.role-pill {
  border: 1px solid rgba(52, 211, 153, 0.28);
  border-radius: 999px;
  color: #6ee7b7;
  display: inline-flex;
  font-size: 0.7rem;
  font-weight: 800;
  padding: 4px 8px;
  text-transform: uppercase;
}

.nav-link {
  border-radius: 12px;
  color: #cde7ff;
  font-weight: 700;
  padding: 10px 12px;
  text-decoration: none;
}

.nav-link:hover,
.nav-link.router-link-active {
  background: rgba(45, 212, 191, 0.12);
  color: #6ee7b7;
}

.dashboard-main {
  min-width: 0;
}

.dashboard-header {
  align-items: center;
  border: 1px solid rgba(45, 212, 191, 0.18);
  border-radius: 18px;
  background: rgba(2, 12, 23, 0.72);
  display: flex;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 24px;
  padding: 22px 24px;
}

.dashboard-header h1 {
  color: #f8fafc;
  margin: 0 0 8px;
}

.dashboard-header p {
  color: #9fb3c8;
  margin: 0;
}

.notification-bell {
  align-items: center;
  background: rgba(15, 23, 42, 0.95);
  border: 1px solid rgba(45, 212, 191, 0.24);
  border-radius: 999px;
  color: #e5eefc;
  display: inline-flex;
  height: 42px;
  justify-content: center;
  position: relative;
  text-decoration: none;
  width: 42px;
}

.notification-badge {
  align-items: center;
  background: #ef4444;
  border-radius: 999px;
  color: white;
  display: flex;
  font-size: 0.72rem;
  font-weight: 900;
  height: 20px;
  justify-content: center;
  min-width: 20px;
  padding: 0 6px;
  position: absolute;
  right: -6px;
  top: -6px;
}

@media (max-width: 860px) {
  .dashboard-layout {
    grid-template-columns: 1fr;
  }

  .sidebar {
    position: static;
  }
}
.nav-link.router-link-active {
  background: linear-gradient(
    90deg,
    rgba(45, 212, 191, 0.18),
    rgba(59, 130, 246, 0.12)
  );

  border: 1px solid rgba(45, 212, 191, 0.28);

  box-shadow:
    inset 0 0 0 1px rgba(255, 255, 255, 0.02),
    0 0 24px rgba(45, 212, 191, 0.08);
}
</style>
