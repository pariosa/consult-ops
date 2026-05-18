<script setup lang="ts">
import { computed } from 'vue';
import { usePermissions } from '~/composables/usePermissions';

const { canManageFinance, canManageAgreements, canProcessTransactions, role } =
  usePermissions();

const navItems = computed(() =>
  [
    {
      label: 'Notifications',
      to: '/notifications',
      show: true,
    },
    {
      label: 'Organization HQ',
      to: '/organization',
      show: true,
    },
    {
      label: 'Members',
      to: '/organization/members',
      show: canManageAgreements.value,
    },
    {
      label: 'Invitations',
      to: '/organization/invitations',
      show: canManageAgreements.value,
    },
    {
      label: 'Operational Finance',
      to: '/organization/finance',
      show: canManageFinance.value,
    },
    {
      label: 'Projects',
      to: '/projects',
      show: true,
    },
    {
      label: 'Clients',
      to: '/clients',
      show: true,
    },

    {
      label: 'Engagements',
      to: '/engagements',
      show: canProcessTransactions.value || canManageAgreements.value,
    },
  ].filter((item) => item.show),
);
</script>

<template>
  <aside class="org-sidebar">
    <div class="sidebar-header">
      <p class="eyebrow">Workspace</p>
      <h2>Consult Ops</h2>
      <span class="role-pill">{{ role || 'member' }}</span>
    </div>

    <nav class="nav-list">
      <NuxtLink
        v-for="item in navItems"
        :key="item.to"
        :to="item.to"
        class="nav-link"
        active-class="active"
      >
        {{ item.label }}
      </NuxtLink>
    </nav>
  </aside>
</template>

<style scoped>
.org-sidebar {
  border: 1px solid rgba(45, 212, 191, 0.18);
  border-radius: 18px;
  background: rgba(2, 12, 23, 0.96);
  color: #e5eefc;
  padding: 20px;
}

.sidebar-header {
  border-bottom: 1px solid rgba(45, 212, 191, 0.18);
  margin-bottom: 16px;
  padding-bottom: 16px;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.7rem;
  font-weight: 900;
  letter-spacing: 0.12em;
  margin: 0 0 8px;
  text-transform: uppercase;
}

h2 {
  color: #f8fafc;
  margin: 0 0 10px;
}

.role-pill {
  border-radius: 999px;
  background: rgba(52, 211, 153, 0.14);
  color: #6ee7b7;
  display: inline-flex;
  font-size: 0.72rem;
  font-weight: 900;
  padding: 6px 10px;
  text-transform: uppercase;
}

.nav-list {
  display: grid;
  gap: 8px;
}

.nav-link {
  border: 1px solid transparent;
  border-radius: 12px;
  color: #cbd5e1;
  font-weight: 800;
  padding: 11px 12px;
  text-decoration: none;
}

.nav-link:hover,
.nav-link.active {
  background: rgba(45, 212, 191, 0.1);
  border-color: rgba(45, 212, 191, 0.28);
  color: #f8fafc;
}
</style>
