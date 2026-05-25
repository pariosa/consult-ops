<script setup lang="ts">
import { usePlatformAdmin } from '~/composables/usePlatformAdmin';

const { getOrganizations, createOrganization } = usePlatformAdmin();

const loading = ref(true);
const saving = ref(false);
const error = ref('');
const success = ref('');
const organizations = ref<any[]>([]);

const organizationForm = ref({
  name: '',
});

async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    organizations.value = await getOrganizations();
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to load organizations.';
  } finally {
    loading.value = false;
  }
}

async function submitOrganization() {
  saving.value = true;
  error.value = '';
  success.value = '';

  try {
    await createOrganization(organizationForm.value);
    success.value = 'Organization created.';
    organizationForm.value.name = '';
    await refresh();
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to create organization.';
  } finally {
    saving.value = false;
  }
}

onMounted(refresh);
</script>

<template>
  <DashboardShell
    title="Platform Organizations"
    subtitle="Create and manage organizations."
  >
    <section v-if="error" class="form-error">{{ error }}</section>
    <section v-if="success" class="success-state">{{ success }}</section>

    <section class="portal-section">
      <p class="eyebrow">Create Organization</p>

      <label>Organization Name</label>
      <input v-model="organizationForm.name" class="form-input" />

      <button
        class="form-button"
        :disabled="saving || !organizationForm.name"
        @click="submitOrganization"
      >
        Create Organization
      </button>
    </section>

    <section class="portal-section">
      <p class="eyebrow">Organizations</p>

      <div v-if="loading">Loading organizations...</div>

      <div v-else class="table-list">
        <NuxtLink
          v-for="org in organizations"
          :key="org.id"
          :to="`/platform/organization/${org.id}`"
          class="table-row"
        >
          <span>{{ org.name }}</span>
          <span>{{ org.slug }}</span>
        </NuxtLink>
      </div>
    </section>
  </DashboardShell>
</template>
