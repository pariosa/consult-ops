<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import DashboardShell from '~/components/DashboardShell.vue';
import UserTypeEditor from '~/components/UserTypeEditor.vue';

definePageMeta({
  middleware: ['role'],
  allowedUserTypes: ['admin'],
});

const route = useRoute();
const user = ref<any | null>(null);
const error = ref('');
const message = ref('');

const userId = computed(() => route.params.id);

const fetchUser = async () => {
  error.value = '';
  message.value = '';

  const res = await fetch(
    `http://127.0.0.1:8000/api/admin/users/${userId.value}`,
  );

  if (!res.ok) {
    error.value = await res.text();
    return;
  }

  user.value = await res.json();
};

const updateUserType = async (payload: { user_type: string }) => {
  if (!user.value) return;

  error.value = '';
  message.value = '';

  const res = await fetch(
    `http://127.0.0.1:8000/api/admin/users/${user.value.id}/type`,
    {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    },
  );

  if (!res.ok) {
    error.value = await res.text();
    return;
  }

  user.value = await res.json();
  message.value = 'User type updated.';
};

onMounted(fetchUser);
</script>

<template>
  <DashboardShell>
    <NuxtLink class="back-link" to="/admin/users">← Back to users</NuxtLink>

    <section v-if="user" class="panel">
      <div class="header">
        <div>
          <p class="eyebrow">User Profile</p>
          <h1>{{ user.name || user.email }}</h1>
          <p>{{ user.email }}</p>
        </div>

        <span class="badge">{{ user.user_type }}</span>
      </div>

      <UserTypeEditor :user-type="user.user_type" @submit="updateUserType" />

      <p v-if="message" class="message">{{ message }}</p>
      <p v-if="error" class="error">{{ error }}</p>
    </section>

    <p v-else-if="error" class="error">{{ error }}</p>
    <p v-else>Loading user...</p>
  </DashboardShell>
</template>

<style scoped>
.back-link {
  display: inline-block;
  margin-bottom: 1rem;
  color: #7dd3fc;
}

.panel {
  padding: 1.5rem;
  border: 1px solid rgba(80, 210, 170, 0.25);
  border-radius: 1.25rem;
  background: rgba(8, 19, 31, 0.82);
}

.header {
  display: flex;
  justify-content: space-between;
  gap: 2rem;
  margin-bottom: 2rem;
}

.eyebrow {
  color: #55d6be;
  text-transform: uppercase;
  letter-spacing: 0.12em;
}

h1 {
  margin: 0;
}

p {
  color: #a8bdd2;
}

.badge {
  align-self: start;
  padding: 0.35rem 0.75rem;
  border-radius: 999px;
  background: rgba(96, 165, 250, 0.16);
  color: #7dd3fc;
}

.message {
  color: #6ee7b7;
}

.error {
  color: #f87171;
}
</style>
