<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useOrganizationClients } from '~/composables/useOrganizationClients';

const {
  getOrganizationClients,
  createOrganizationClient,
  createVerifiedClientParty,
} = useOrganizationClients();

const organizationId = 1;
const clients = ref<any[]>([]);
const loading = ref(false);
const saving = ref(false);
const error = ref('');
const success = ref('');

const form = ref({
  name: '',
  email: '',
  company: '',
  phone: '',
});

async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    clients.value = await getOrganizationClients(organizationId);
  } catch (err: any) {
    error.value = err?.message || 'Failed to load clients.';
  } finally {
    loading.value = false;
  }
}

async function submitClient() {
  saving.value = true;
  error.value = '';
  success.value = '';

  try {
    await createOrganizationClient(organizationId, {
      name: form.value.name,
      email: form.value.email,
      company: form.value.company || undefined,
      phone: form.value.phone || undefined,
    });

    success.value = 'Client created.';
    form.value = {
      name: '',
      email: '',
      company: '',
      phone: '',
    };

    await refresh();
  } catch (err: any) {
    error.value = err?.message || 'Failed to create client.';
  } finally {
    saving.value = false;
  }
}

async function verifyClientParty(clientId: number) {
  saving.value = true;
  error.value = '';
  success.value = '';

  try {
    await createVerifiedClientParty(organizationId, clientId);
    success.value = 'Verified client party created.';
    await refresh();
  } catch (err: any) {
    error.value = err?.message || 'Failed to create verified party.';
  } finally {
    saving.value = false;
  }
}

onMounted(refresh);
</script>

<template>
  <DashboardShell
    title="Organization Clients"
    subtitle="Manage client accounts and verified client parties."
  >
    <section class="portal-section">
      <p class="eyebrow">Create Client</p>

      <div class="form-grid">
        <label>Name</label>
        <input v-model="form.name" class="form-input" />

        <label>Email</label>
        <input v-model="form.email" class="form-input" />

        <label>Company</label>
        <input v-model="form.company" class="form-input" />

        <label>Phone</label>
        <input v-model="form.phone" class="form-input" />

        <button
          class="form-button"
          :disabled="saving || !form.name || !form.email"
          @click="submitClient"
        >
          Create Client
        </button>
      </div>
    </section>

    <section v-if="error" class="form-error">{{ error }}</section>
    <section v-if="success" class="success-state">{{ success }}</section>

    <section class="portal-section">
      <p class="eyebrow">Clients</p>

      <p v-if="loading">Loading clients...</p>

      <div v-else-if="!clients.length" class="empty-state">No clients yet.</div>

      <div v-else class="table-list">
        <div class="table-row table-row--header">
          <span>Name</span>
          <span>Email</span>
          <span>Company</span>
          <span>Phone</span>
          <span>Actions</span>
        </div>

        <div v-for="client in clients" :key="client.id" class="table-row">
          <span>{{ client.name }}</span>
          <span>{{ client.email }}</span>
          <span>{{ client.company || '—' }}</span>
          <span>{{ client.phone || '—' }}</span>

          <span class="actions">
            <button
              class="form-button secondary"
              :disabled="saving"
              @click="verifyClientParty(client.id)"
            >
              Create Verified Party
            </button>
          </span>
        </div>
      </div>
    </section>
  </DashboardShell>
</template>
