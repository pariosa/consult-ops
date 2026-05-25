<script setup lang="ts">
const route = useRoute();
const config = useRuntimeConfig();

const user = ref<any>(null);
const memberships = ref<any[]>([]);
const error = ref('');
const success = ref('');

const userId = computed(() => Number(route.params.id));

function authHeaders() {
  return {
    Authorization: `Bearer ${localStorage.getItem('token')}`,
  };
}

async function refresh() {
  error.value = '';

  try {
    user.value = await $fetch(
      `${config.public.apiBase}/api/admin/users/${userId.value}`,
      {
        headers: authHeaders(),
      },
    );

    memberships.value = await $fetch(
      `${config.public.apiBase}/api/admin/users/${userId.value}/memberships`,
      { headers: authHeaders() },
    );
  } catch (err: any) {
    error.value = err?.data?.error || err?.message || 'Failed to load user.';
  }
}

async function action(path: string, message: string) {
  error.value = '';
  success.value = '';

  try {
    await $fetch(`${config.public.apiBase}${path}`, {
      method: path.includes('/sessions') ? 'DELETE' : 'POST',
      headers: authHeaders(),
    });

    success.value = message;
    await refresh();
  } catch (err: any) {
    error.value = err?.data?.error || err?.message || 'Action failed.';
  }
}

onMounted(refresh);
</script>

<template>
  <DashboardShell
    title="Platform User Detail"
    subtitle="Inspect memberships and run admin actions."
  >
    <section v-if="error" class="form-error">{{ error }}</section>
    <section v-if="success" class="success-state">{{ success }}</section>

    <section v-if="user" class="portal-section">
      <p class="eyebrow">User</p>
      <h2>{{ user.email }}</h2>
      <p>{{ user.name }}</p>
      <p>{{ user.user_type }}</p>

      <div class="actions">
        <button
          class="form-button"
          @click="
            action(
              `/api/admin/users/${userId}/force-password-reset`,
              'Password reset forced.',
            )
          "
        >
          Force Password Reset
        </button>

        <button
          class="form-button"
          @click="
            action(`/api/admin/users/${userId}/sessions`, 'Sessions revoked.')
          "
        >
          Revoke Sessions
        </button>
      </div>
    </section>

    <section class="portal-section">
      <p class="eyebrow">Memberships</p>

      <div v-if="!memberships.length" class="empty-state">
        No memberships found.
      </div>

      <div v-else class="table-list">
        <div
          v-for="membership in memberships"
          :key="membership.organization_id"
          class="table-row"
        >
          <span>{{
            membership.organization_name || membership.organization_id
          }}</span>
          <span>{{ membership.role }}</span>
          <span>{{ membership.status }}</span>
        </div>
      </div>
    </section>
  </DashboardShell>
</template>
