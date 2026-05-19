<script setup lang="ts">
import { computed } from 'vue';
import { useAuth } from '~/composables/useAuth';

const { authUser } = useAuth();

const displayName = computed(
  () => authUser.value?.name || authUser.value?.email || 'Current User',
);
const displayEmail = computed(() => authUser.value?.email || '—');
const displayRole = computed(
  () => authUser.value?.user_type || authUser.value?.role || 'member',
);
</script>

<template>
  <DashboardShell
    title="Profile"
    subtitle="Your account and workspace identity."
  >
    <section class="portal-section">
      <p class="eyebrow">Account</p>
      <h2>{{ displayName }}</h2>

      <div class="profile-grid">
        <div>
          <span>Email: </span>
          <strong>{{ displayEmail }}</strong>
        </div>

        <div>
          <span>Role: </span>
          <strong>{{ displayRole }}</strong>
        </div>
      </div>
    </section>
  </DashboardShell>
</template>

<style>
.profile-grid {
  color: antiquewhite;
}
.portal-section > h2 {
  color: antiquewhite;
}
</style>
