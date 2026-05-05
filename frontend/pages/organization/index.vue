<!-- frontend/pages/organization/index.vue -->
<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useApi } from '~/composables/useApi';
import { useOrganizationStore } from '~/stores/organization';

definePageMeta({
  middleware: ['role'],
  allowedUserTypes: ['admin', 'consultant', 'client'],
});

const { apiFetch } = useApi();
const orgStore = useOrganizationStore();

const projects = ref<any[]>([]);
const clients = ref<any[]>([]);
const contracts = ref<any[]>([]);
const invoices = ref<any[]>([]);
const payments = ref<any[]>([]);

const loading = ref(true);
const error = ref('');

async function loadOrganizationOverview() {
  loading.value = true;
  error.value = '';

  try {
    await orgStore.fetchCurrentOrganization();
    await orgStore.fetchMembers();

    const orgId = orgStore.organization?.id;

    if (!orgId) {
      throw new Error('No organization found for this user.');
    }

    const [projectsRes, clientsRes, contractsRes, invoicesRes, paymentsRes] =
      await Promise.all([
        apiFetch<any[]>(`/api/organizations/${orgId}/projects`),
        apiFetch<any[]>(`/api/organizations/${orgId}/clients`),
        apiFetch<any[]>(`/api/organizations/${orgId}/contracts`),
        apiFetch<any[]>(`/api/organizations/${orgId}/invoices`),
        apiFetch<any[]>(`/api/organizations/${orgId}/payments`),
      ]);

    projects.value = projectsRes || [];
    clients.value = clientsRes || [];
    contracts.value = contractsRes || [];
    invoices.value = invoicesRes || [];
    payments.value = paymentsRes || [];
  } catch (err: any) {
    console.error('Organization overview failed:', err);
    error.value =
      orgStore.error ||
      err?.data?.message ||
      err?.message ||
      'Failed to load organization.';
  } finally {
    loading.value = false;
  }
}

onMounted(loadOrganizationOverview);
</script>

<template>
  <DashboardShell title="Organization" subtitle="Manage your company workspace">
    <section v-if="loading" class="portal-section">
      Loading organization...
    </section>

    <section v-else-if="error" class="form-error">
      {{ error }}
    </section>

    <section v-else class="dashboard-grid">
      <NuxtLink to="/organization" class="kpi-link">
        <KPITile
          label="Organization"
          :value="orgStore.organization?.name || '—'"
        />
      </NuxtLink>

      <NuxtLink to="/organization/members" class="kpi-link">
        <KPITile label="Members" :value="orgStore.memberCount" />
      </NuxtLink>

      <NuxtLink to="/organization/projects" class="kpi-link">
        <KPITile label="Projects" :value="projects.length" />
      </NuxtLink>

      <NuxtLink to="/clients" class="kpi-link">
        <KPITile label="Clients" :value="clients.length" />
      </NuxtLink>

      <!-- <NuxtLink to="/contracts" class="kpi-link"> -->
      <KPITile label="Contracts" :value="contracts.length" />
      <!-- </NuxtLink> -->

      <!-- <NuxtLink to="/invoices" class="kpi-link"> -->
      <KPITile label="Invoices" :value="invoices.length" />
      <!-- </NuxtLink> -->

      <!-- <NuxtLink to="/payments" class="kpi-link"> -->
      <KPITile label="Payments" :value="payments.length" />
      <!-- </NuxtLink> -->
    </section>
  </DashboardShell>
</template>

<style>
.shell-header h1 {
  color: white;
}

.shell-header p {
  color: #9fb3c8;
}
</style>
