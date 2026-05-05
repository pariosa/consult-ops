<script setup lang="ts">
import { ref, onMounted } from 'vue';
import DashboardShell from '~/components/DashboardShell.vue';

definePageMeta({
  middleware: ['role'],
  allowedUserTypes: ['admin', 'consultant', 'client'],
});

const user = ref<any | null>(null);

onMounted(() => {
  const raw = localStorage.getItem('auth_user');
  user.value = raw ? JSON.parse(raw) : null;
});
</script>

<template>
  <DashboardShell title="Profile" subtitle="Manage your account details.">
    <section class="panel">
      <p class="eyebrow">Profile</p>
      <h1>My Account</h1>

      <dl v-if="user">
        <div>
          <dt>Email</dt>
          <dd>{{ user.email }}</dd>
        </div>

        <div>
          <dt>User ID</dt>
          <dd>{{ user.id || user.user_id || '—' }}</dd>
        </div>

        <div>
          <dt>User Type</dt>
          <dd>{{ user.user_type || user.role || '—' }}</dd>
        </div>
      </dl>

      <p v-else>No user session found.</p>
    </section>
  </DashboardShell>
</template>
<style scoped>
.panel {
  padding: 1.5rem;
  border: 1px solid rgba(80, 210, 170, 0.25);
  border-radius: 1.25rem;
  background: rgba(8, 19, 31, 0.82);
}

.eyebrow {
  color: #55d6be;
  text-transform: uppercase;
  letter-spacing: 0.12em;
}

dl {
  display: grid;
  gap: 1rem;
}

dt {
  color: #9fb3c8;
}

dd {
  margin: 0.25rem 0 0;
  color: white;
}
</style>
