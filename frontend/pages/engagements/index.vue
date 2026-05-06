<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { useRouter } from 'nuxt/app';
import { useEngagements } from '~/composables/useEngagements';
import { useProjects } from '~/composables/useProjects';
import { useClients } from '~/composables/useClients';
import { useApi } from '~/composables/useApi';

const router = useRouter();
const { get } = useApi();
const { getProjectEngagements, createEngagement } = useEngagements();
const { getOrganizationProjects, createProject } = useProjects();
const { getOrganizationClients, createClient } = useClients();
const organization = ref<any>(null);
const organizationId = computed(() => organization.value?.id);
const clients = ref<any[]>([]);
const projects = ref<any[]>([]);
const selectedClientId = ref<number | null>(null);

const selectedProjectId = ref<number | null>(null);
const engagements = ref<any[]>([]);

const loading = ref(false);
const error = ref('');
const showCreateEngagement = ref(false);
const showCreateProject = ref(false);
const showCreateClient = ref(false);

const clientForm = ref({
  organization_id: null,
  name: '',
  email: '',
  tax_id: '',
  phone: '',
  company_name: '',
  address: '',
  city: '',
  state: '',
  zip: '',
  country: 'US',
});

const engagementForm = ref({
  contractor_name: '',
  contractor_email: '',
  role: 'Full-stack Developer',
  title: '',
  scope_of_work: '',
  deliverables: '',
  repo_url: '',
  amount_cents: 0,
  due_date: '',
});

const projectForm = ref({
  client_id: null,
  name: '',
  start_date: '',
  end_date: '',
  description: '',
});

const hasProjects = computed(() => projects.value.length > 0);

async function loadProjects() {
  loading.value = true;
  error.value = '';

  try {
    organization.value = await get('/api/me/organization');

    if (!organization.value?.id) {
      projects.value = [];
      clients.value = [];
      engagements.value = [];
      error.value = 'No organization found for this user.';
      return;
    }

    clients.value = await getOrganizationClients(organization.value.id);
    projects.value = await getOrganizationProjects(organization.value.id);

    if (!selectedClientId.value && clients.value.length) {
      selectedClientId.value = clients.value[0].id;
    }

    if (!selectedProjectId.value && projects.value.length) {
      selectedProjectId.value = projects.value[0].id;
    }

    await loadEngagements();
  } catch (err: any) {
    projects.value = [];
    clients.value = [];
    engagements.value = [];
    error.value = err?.message || 'Failed to load organization workspace.';
  } finally {
    loading.value = false;
  }
}

async function loadEngagements() {
  if (!selectedProjectId.value) {
    engagements.value = [];
    return;
  }

  try {
    engagements.value = await getProjectEngagements(selectedProjectId.value);
  } catch (err: any) {
    engagements.value = [];
    console.error('Failed to load project engagements', err);
  }
}

async function submitProject(payload: any) {
  if (!organizationId.value) {
    error.value = 'No organization found for this user.';
    return;
  }

  if (!payload.client_id) {
    error.value = 'Select or create a client before creating a project.';
    return;
  }

  try {
    const created = await createProject(organizationId.value, payload);

    projects.value.unshift(created);
    selectedProjectId.value = created.id;

    showCreateProject.value = false;
    showCreateEngagement.value = true;

    await loadEngagements();
  } catch (err: any) {
    error.value = err?.message || 'Failed to create project.';
  }
}
async function submitClient(payload: any) {
  if (!organizationId.value) {
    error.value = 'No organization found for this user.';
    return;
  }

  try {
    const created = await createClient(organizationId.value, {
      ...payload,
      organization_id: organizationId.value,
    });

    clients.value.unshift(created);
    selectedClientId.value = created.id;

    showCreateClient.value = false;
    showCreateProject.value = true;
  } catch (err: any) {
    error.value = err?.message || 'Failed to create client.';
  }
}
async function submitEngagement() {
  if (!selectedProjectId.value) {
    error.value = 'Select a project before creating an engagement.';
    return;
  }

  try {
    const created = await createEngagement(
      selectedProjectId.value,
      engagementForm.value,
    );

    await router.push(`/engagements/${created.id}`);
  } catch (err: any) {
    error.value = err?.message || 'Failed to create engagement.';
  }
}

watch(selectedProjectId, loadEngagements);

onMounted(loadProjects);
</script>

<template>
  <DashboardShell
    title="Engagements"
    subtitle="Create and manage contractor workflows."
  >
    <section v-if="error" class="form-error">
      {{ error }}
    </section>
    <section class="portal-section">
      <div class="section-header">
        <div>
          <p class="eyebrow">Clients</p>
          <h2>Organization Clients</h2>
          <p>Select a client before creating a project.</p>
        </div>

        <button
          class="form-button"
          @click="showCreateClient = !showCreateClient"
        >
          {{ showCreateClient ? 'Cancel Client' : 'Add Client' }}
        </button>
      </div>

      <label v-if="clients.length" class="form-label">
        Client
        <select v-model.number="selectedClientId" class="form-input">
          <option v-for="client in clients" :key="client.id" :value="client.id">
            {{ client.name }}{{ client.company ? ` — ${client.company}` : '' }}
          </option>
        </select>
      </label>

      <div v-else class="empty-state">
        <h3>No clients yet</h3>
        <p>Create a client first, then attach projects and engagements.</p>
      </div>
    </section>

    <section v-if="showCreateClient" class="portal-section">
      <ClientForm v-model="clientForm" @submit="submitClient" />
    </section>
    <section class="portal-section">
      <div class="section-header">
        <div>
          <p class="eyebrow">Workflow Console</p>
          <h2>Software Engagements</h2>
          <p>Choose a project, then create contractor engagements.</p>
        </div>

        <button
          class="form-button"
          @click="showCreateProject = !showCreateProject"
        >
          {{ showCreateProject ? 'Cancel Project' : 'Add Project' }}
        </button>
      </div>

      <div v-if="loading">Loading projects...</div>

      <div v-else-if="hasProjects">
        <label class="form-label">
          Project
          <select v-model.number="selectedProjectId" class="form-input">
            <option
              v-for="project in projects"
              :key="project.id"
              :value="project.id"
            >
              {{ project.name }}
            </option>
          </select>
        </label>

        <button
          class="form-button"
          @click="showCreateEngagement = !showCreateEngagement"
        >
          {{ showCreateEngagement ? 'Cancel Engagement' : 'Create Engagement' }}
        </button>
      </div>

      <div v-else class="empty-state">
        <h3>No projects yet</h3>
        <p>Create a project first, then attach engagements to it.</p>

        <button class="form-button" @click="showCreateProject = true">
          Add Project
        </button>
      </div>
    </section>

    <section v-if="showCreateProject" class="portal-section">
      <ProjectForm
        v-model="projectForm"
        :client-id="selectedClientId"
        @submit="submitProject"
      />
    </section>

    <section
      v-if="showCreateEngagement && selectedProjectId"
      class="portal-section"
    >
      <SoftwareEngagementForm
        v-model="engagementForm"
        @submit="submitEngagement"
      />
    </section>

    <section class="portal-section">
      <div class="section-header">
        <h2>Active Engagements</h2>

        <button
          class="form-button secondary"
          :disabled="!selectedProjectId"
          @click="loadEngagements"
        >
          Refresh
        </button>
      </div>

      <p v-if="!selectedProjectId">Select or create a project first.</p>

      <p v-else-if="!engagements.length">
        No engagements found for this project.
      </p>

      <div
        v-for="engagement in engagements"
        :key="engagement.id"
        class="ops-card engagement-row"
      >
        <div>
          <h3>{{ engagement.title }}</h3>
          <p>
            {{ engagement.contractor_name }} — {{ engagement.contractor_email }}
          </p>
          <p>Status: {{ engagement.status }}</p>
          <p>Platform fee: {{ engagement.platform_fee_status }}</p>
        </div>

        <NuxtLink
          class="form-button link-button"
          :to="`/engagements/${engagement.id}`"
        >
          Open Tracker
        </NuxtLink>
      </div>
    </section>
  </DashboardShell>
</template>
