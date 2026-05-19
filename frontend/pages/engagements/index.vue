<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { useRouter } from 'nuxt/app';
import { useEngagements } from '~/composables/useEngagements';
import { useProjects } from '~/composables/useProjects';
import { useClients } from '~/composables/useClients';
import { useApi } from '~/composables/useApi';
import ConsultingEngagementForm from '~/components/EngagementForms/ConsultingEngagementForm.vue';

import CreativeEngagementForm from '~/components/EngagementForms/CreativeEngagementForm.vue';

import FieldServicesEngagementForm from '~/components/EngagementForms/FieldServicesEngagementForm.vue';

import MaintenanceEngagementForm from '~/components/EngagementForms/MaintenanceEngagementForm.vue';

import RentalEngagementForm from '~/components/EngagementForms/RentalEngagementForm.vue';

import SoftwareEngagementForm from '~/components/EngagementForms/SoftwareEngagementForm.vue';

const router = useRouter();
const { get } = useApi();
const { getProjectEngagements, createEngagement } = useEngagements();
const { getOrganizationProjects, createProject } = useProjects();
const { getOrganizationClients, createClient } = useClients();

const organization = ref<any>(null);
const organizationId = computed(() => organization.value?.id);

const clients = ref<any[]>([]);
const projects = ref<any[]>([]);
const engagements = ref<any[]>([]);

const selectedClientId = ref<number | null>(null);
const selectedProjectId = ref<number | null>(null);
const selectedEngagementType = ref('software');

const loading = ref(false);
const saving = ref(false);
const error = ref('');
const success = ref('');

const showCreateClient = ref(false);
const showCreateProject = ref(false);
const showCreateEngagement = ref(false);

const engagementTypes = [
  { value: 'software', label: 'Software Development' },
  { value: 'consulting', label: 'Consulting / Advisory' },
  { value: 'field_services', label: 'Field Services' },
  { value: 'creative', label: 'Creative / Content' },
  { value: 'maintenance', label: 'Maintenance / Retainer' },
  { value: 'rental', label: 'Equipment / Venue Rental' },
];

const engagementTypeLabels: Record<string, string> = {
  software: 'Software Development',
  consulting: 'Consulting Services',
  field_services: 'Field Services',
  creative: 'Creative Services',
  maintenance: 'Maintenance Retainer',
  rental: 'Equipment / Venue Rental',
};

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

const projectForm = ref({
  client_id: null,
  name: '',
  start_date: '',
  end_date: '',
  description: '',
});

const engagementForm = ref({
  contractor_name: '',
  contractor_email: '',
  role: '',
  title: '',
  scope_of_work: '',
  deliverables: '',
  repo_url: '',
  amount_cents: 0,
  due_date: '',
});

const selectedClient = computed(() =>
  clients.value.find((client) => client.id === selectedClientId.value),
);

const filteredProjects = computed(() =>
  selectedClientId.value
    ? projects.value.filter(
        (project) => project.client_id === selectedClientId.value,
      )
    : projects.value,
);

const selectedProject = computed(() =>
  projects.value.find((project) => project.id === selectedProjectId.value),
);

function getEngagementTypeLabel(type?: string) {
  if (!type) return 'General Engagement';

  return (
    engagementTypeLabels[type] ||
    type.replaceAll('_', ' ').replace(/\b\w/g, (c) => c.toUpperCase())
  );
}

function applyTypeDefaults() {
  const defaults: Record<string, Partial<typeof engagementForm.value>> = {
    software: {
      role: 'Full-stack Developer',
      title: 'Software Delivery Engagement',
      scope_of_work: 'Build, test, and deliver agreed software features.',
      deliverables:
        'Source code, deployment notes, documentation, and passing tests.',
    },
    consulting: {
      role: 'Consultant',
      title: 'Consulting Engagement',
      scope_of_work:
        'Provide strategic guidance, analysis, and recommendations.',
      deliverables:
        'Discovery notes, recommendations, implementation plan, and final report.',
    },
    field_services: {
      role: 'Field Specialist',
      title: 'Field Services Engagement',
      scope_of_work: 'Perform on-site or operational field service work.',
      deliverables:
        'Completed service checklist, findings, photos, and completion report.',
    },
    creative: {
      role: 'Creative Contractor',
      title: 'Creative Services Engagement',
      scope_of_work: 'Produce creative assets according to the approved brief.',
      deliverables:
        'Draft assets, revision rounds, final files, and usage notes.',
    },
    maintenance: {
      role: 'Maintenance Contractor',
      title: 'Maintenance Retainer',
      scope_of_work:
        'Provide ongoing maintenance, support, and issue resolution.',
      deliverables:
        'Monthly support summary, resolved tickets, and maintenance log.',
    },
  };

  engagementForm.value = {
    ...engagementForm.value,
    ...defaults[selectedEngagementType.value],
  };
}

async function loadWorkspace() {
  loading.value = true;
  error.value = '';

  try {
    organization.value = await get('/api/me/organization');

    if (!organization.value?.id) {
      error.value = 'No organization found for this user.';
      return;
    }

    clients.value = await getOrganizationClients(organization.value.id);
    projects.value = await getOrganizationProjects(organization.value.id);

    if (!selectedClientId.value && clients.value.length) {
      selectedClientId.value = clients.value[0].id;
    }

    if (!selectedProjectId.value && filteredProjects.value.length) {
      selectedProjectId.value = filteredProjects.value[0].id;
    }

    await loadEngagements();
  } catch (err: any) {
    error.value = err?.message || 'Failed to load engagement workspace.';
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
  } catch (err) {
    engagements.value = [];
    console.error('Failed to load project engagements', err);
  }
}

async function submitClient(payload: any) {
  if (!organizationId.value) return;

  saving.value = true;
  error.value = '';
  success.value = '';

  try {
    const created = await createClient(organizationId.value, {
      ...payload,
      organization_id: organizationId.value,
    });

    clients.value.unshift(created);
    selectedClientId.value = created.id;
    showCreateClient.value = false;
    showCreateProject.value = true;
    success.value = 'Client created. Now create a project for this client.';
  } catch (err: any) {
    error.value = err?.message || 'Failed to create client.';
  } finally {
    saving.value = false;
  }
}

async function submitProject(payload: any) {
  if (!organizationId.value) return;

  saving.value = true;
  error.value = '';
  success.value = '';

  try {
    const created = await createProject(organizationId.value, {
      ...payload,
      client_id: selectedClientId.value || payload.client_id,
    });

    projects.value.unshift(created);
    selectedProjectId.value = created.id;
    showCreateProject.value = false;
    showCreateEngagement.value = true;
    success.value = 'Project created. Now create an engagement.';
    await loadEngagements();
  } catch (err: any) {
    error.value = err?.message || 'Failed to create project.';
  } finally {
    saving.value = false;
  }
}

async function submitEngagement(payload?: any) {
  if (!selectedProjectId.value) {
    error.value = 'Select a project before creating an engagement.';
    return;
  }

  saving.value = true;
  error.value = '';

  const body = payload ?? engagementForm.value;

  try {
    const created = await createEngagement(selectedProjectId.value, {
      contractor_name: body.contractor_name,
      contractor_email: body.contractor_email,
      role: body.role,
      title: body.title,
      scope_of_work: body.scope_of_work,
      deliverables: body.deliverables,
      repo_url: body.repo_url,
      amount_cents: Number(body.amount_cents ?? 0),
      due_date: body.due_date || null,
      engagement_type: selectedEngagementType.value,
    });

    await router.push(`/engagements/${created.id}`);
  } catch (err: any) {
    error.value = err?.message || 'Failed to create engagement.';
  } finally {
    saving.value = false;
  }
}

watch(selectedClientId, () => {
  selectedProjectId.value = filteredProjects.value[0]?.id ?? null;
  loadEngagements();
});

watch(selectedProjectId, loadEngagements);
watch(selectedEngagementType, applyTypeDefaults);

onMounted(async () => {
  applyTypeDefaults();
  await loadWorkspace();
});
</script>

<template>
  <DashboardShell
    title="Engagements"
    subtitle="Create and manage client projects, contractor work, agreements, and payout workflows."
  >
    <section v-if="error" class="form-error">{{ error }}</section>
    <section v-if="success" class="success-state">{{ success }}</section>

    <section class="workspace-grid">
      <article class="portal-section">
        <div class="section-header">
          <div>
            <p class="eyebrow">Step 1</p>
            <h2>Client</h2>
            <p>Select the client this work belongs to.</p>
          </div>

          <button
            class="form-button secondary"
            @click="showCreateClient = !showCreateClient"
          >
            {{ showCreateClient ? 'Cancel' : 'Add Client' }}
          </button>
        </div>

        <div v-if="clients.length" class="selection-list">
          <button
            v-for="client in clients"
            :key="client.id"
            class="selection-card"
            :class="{ selected: client.id === selectedClientId }"
            @click="selectedClientId = client.id"
          >
            <span class="star">{{
              client.id === selectedClientId ? '★' : '☆'
            }}</span>
            <strong>{{ client.name }}</strong>
            <small>{{ client.email }}</small>
          </button>
        </div>

        <div v-else class="empty-state">No clients yet.</div>
      </article>

      <article class="portal-section">
        <div class="section-header">
          <div>
            <p class="eyebrow">Step 2</p>
            <h2>Project</h2>
            <p>Select or create a project for the selected client.</p>
          </div>

          <button
            class="form-button secondary"
            :disabled="!selectedClientId"
            @click="showCreateProject = !showCreateProject"
          >
            {{ showCreateProject ? 'Cancel' : 'Add Project' }}
          </button>
        </div>

        <div v-if="filteredProjects.length" class="selection-list">
          <button
            v-for="project in filteredProjects"
            :key="project.id"
            class="selection-card"
            :class="{ selected: project.id === selectedProjectId }"
            @click="selectedProjectId = project.id"
          >
            <span class="star">{{
              project.id === selectedProjectId ? '★' : '☆'
            }}</span>
            <strong>{{ project.name }}</strong>
            <small>{{ project.description || 'No description' }}</small>
          </button>
        </div>

        <div v-else class="empty-state">No projects for this client yet.</div>
      </article>
    </section>

    <section v-if="showCreateClient" class="portal-section">
      <ClientForm v-model="clientForm" @submit="submitClient" />
    </section>

    <section v-if="showCreateProject" class="portal-section">
      <ProjectForm
        v-model="projectForm"
        :clients="clients"
        @submit="submitProject"
      />
    </section>

    <section class="portal-section">
      <div class="section-header">
        <div>
          <p class="eyebrow">Step 3</p>
          <h2>Create Engagement</h2>
          <p>
            {{ selectedClient?.name || 'Choose a client' }}
            <span v-if="selectedProject"> / {{ selectedProject.name }}</span>
          </p>
        </div>

        <button
          class="form-button"
          :disabled="!selectedProjectId"
          @click="showCreateEngagement = !showCreateEngagement"
        >
          {{ showCreateEngagement ? 'Cancel Engagement' : 'Create Engagement' }}
        </button>
      </div>

      <label class="form-label">Engagement Type</label>
      <select v-model="selectedEngagementType" class="form-input">
        <option
          v-for="type in engagementTypes"
          :key="type.value"
          :value="type.value"
        >
          {{ type.label }}
        </option>
      </select>
    </section>

    <section
      v-if="showCreateEngagement && selectedProjectId"
      class="portal-section"
    >
      <SoftwareEngagementForm
        v-if="selectedEngagementType === 'software'"
        v-model="engagementForm"
        :projects="projects"
        @submit="submitEngagement"
      />

      <ConsultingEngagementForm
        v-else-if="selectedEngagementType === 'consulting'"
        v-model="engagementForm"
        :projects="projects"
        @submit="submitEngagement"
      />

      <FieldServicesEngagementForm
        v-else-if="selectedEngagementType === 'field_services'"
        v-model="engagementForm"
        :projects="projects"
        @submit="submitEngagement"
      />

      <CreativeEngagementForm
        v-else-if="selectedEngagementType === 'creative'"
        v-model="engagementForm"
        :projects="projects"
        @submit="submitEngagement"
      />

      <MaintenanceEngagementForm
        v-else-if="selectedEngagementType === 'maintenance'"
        v-model="engagementForm"
        :projects="projects"
        @submit="submitEngagement"
      />
      <RentalEngagementForm
        v-else-if="selectedEngagementType === 'rental'"
        v-model="engagementForm"
        :projects="projects"
        @submit="submitEngagement"
      />
    </section>

    <section class="portal-section">
      <div class="section-header">
        <div>
          <p class="eyebrow">Active Work</p>
          <h2>{{ getEngagementTypeLabel() }}</h2>
        </div>

        <button
          class="form-button secondary"
          :disabled="!selectedProjectId"
          @click="loadEngagements"
        >
          Refresh
        </button>
      </div>

      <p v-if="loading">Loading workspace...</p>
      <p v-else-if="!selectedProjectId">Select or create a project first.</p>
      <div v-else-if="!engagements.length" class="empty-state">
        No engagements found for this project.
      </div>

      <div v-else class="engagement-list">
        <article
          v-for="engagement in engagements"
          :key="engagement.id"
          class="ops-card engagement-row"
        >
          <div>
            <p class="eyebrow">{{ engagement.status }}</p>
            <h3>{{ engagement.title }}</h3>
            <p>
              {{ engagement.contractor_name }} —
              {{ engagement.contractor_email }}
            </p>
            <p>
              Platform fee: {{ engagement.platform_fee_status || 'pending' }}
            </p>
          </div>

          <div class="engagement-actions">
            <NuxtLink
              class="form-button secondary"
              :to="`/engagements/${engagement.id}`"
            >
              Open Tracker
            </NuxtLink>

            <NuxtLink
              class="form-button secondary"
              :to="`/engagements/${engagement.id}/agreements`"
            >
              Agreements
            </NuxtLink>

            <NuxtLink
              class="form-button secondary"
              :to="`/engagements/${engagement.id}/transactions`"
            >
              Transactions
            </NuxtLink>
          </div>
        </article>
      </div>
    </section>
  </DashboardShell>
</template>

<style scoped>
.workspace-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 18px;
}

.portal-section {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 18px;
  background: linear-gradient(
    180deg,
    rgba(15, 23, 42, 0.96),
    rgba(2, 12, 23, 0.96)
  );
  color: #e5eefc;
  padding: 24px;
}

.section-header {
  align-items: flex-start;
  display: flex;
  gap: 16px;
  justify-content: space-between;
  margin-bottom: 18px;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.12em;
  margin: 0 0 8px;
  text-transform: uppercase;
}

h2,
h3,
p {
  color: #e5eefc;
}

.selection-list {
  display: grid;
  gap: 10px;
}

.selection-card {
  align-items: flex-start;
  background: rgba(8, 31, 42, 0.7);
  border: 1px solid rgba(45, 212, 191, 0.18);
  border-radius: 14px;
  color: #e5eefc;
  cursor: pointer;
  display: grid;
  gap: 4px;
  grid-template-columns: auto 1fr;
  padding: 14px;
  text-align: left;
}

.selection-card strong,
.selection-card small {
  grid-column: 2;
}

.selection-card.selected {
  border-color: rgba(96, 165, 250, 0.64);
  background: rgba(30, 64, 175, 0.2);
}

.star {
  color: #facc15;
  font-size: 1rem;
  grid-row: 1 / span 2;
}

.engagement-list {
  display: grid;
  gap: 14px;
}

.engagement-row {
  align-items: center;
  border: 1px solid rgba(45, 212, 191, 0.18);
  border-radius: 16px;
  background: rgba(8, 31, 42, 0.7);
  display: flex;
  justify-content: space-between;
  gap: 16px;
  padding: 18px;
}

.engagement-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  justify-content: flex-end;
}

.form-input {
  background: rgba(15, 23, 42, 0.95);
  border: 1px solid rgba(45, 212, 191, 0.28);
  border-radius: 12px;
  color: #e5eefc;
  padding: 12px;
  width: 100%;
}

.form-button {
  background: linear-gradient(90deg, #60a5fa, #34d399);
  border: 0;
  border-radius: 12px;
  color: #020617;
  cursor: pointer;
  font-weight: 900;
  padding: 11px 16px;
  text-decoration: none;
}

.form-button.secondary {
  background: rgba(15, 23, 42, 0.95);
  border: 1px solid rgba(45, 212, 191, 0.32);
  color: #e5eefc;
}

.form-error,
.success-state,
.empty-state {
  border-radius: 14px;
  padding: 16px;
}

.form-error {
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
}

.success-state {
  background: rgba(20, 83, 45, 0.24);
  color: #bbf7d0;
}

.empty-state {
  border: 1px dashed rgba(45, 212, 191, 0.28);
  color: #cbd5e1;
}

@media (max-width: 960px) {
  .workspace-grid {
    grid-template-columns: 1fr;
  }

  .engagement-row {
    align-items: stretch;
    flex-direction: column;
  }

  .engagement-actions {
    justify-content: flex-start;
  }
}
</style>
