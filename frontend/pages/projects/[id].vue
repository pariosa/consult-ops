<script setup lang="ts">
import { ref } from 'vue';
import { useProjects } from '~/composables/useProjects';
import { useEngagements } from '~/composables/useEngagements';

const route = useRoute();
const router = useRouter();

const projectId = Number(route.params.id);

const { getProject } = useProjects();
const { getProjectEngagements } = useEngagements();

const project = ref<any>(null);
const engagements = ref<any[]>([]);
const loading = ref(true);
const error = ref('');

async function load() {
  loading.value = true;
  error.value = '';

  try {
    project.value = await getProject(projectId);
    engagements.value = await getProjectEngagements(projectId);
  } catch (err: any) {
    error.value =
      err?.response?.data?.message ||
      err?.message ||
      `Project #${projectId} could not be loaded.`;
  } finally {
    loading.value = false;
  }
}

function goToNewEngagement() {
  router.push(`/projects/engagements/new?projectId=${projectId}`);
}

function goToEngagement(id: number) {
  router.push(`/engagements/${id}`);
}

await load();
</script>

<template>
  <DashboardShell
    :title="project?.name || `Project #${projectId}`"
    subtitle="Manage engagements, contractors, and delivery for this project."
  >
    <section v-if="loading" class="portal-section">Loading project...</section>

    <section v-else-if="error" class="form-error">
      {{ error }}
    </section>

    <section v-else class="portal-section">
      <div class="section-header">
        <div>
          <p class="eyebrow">Project</p>
          <h2>Engagements</h2>
          <p>
            {{
              project?.description || 'Create and manage work for this project.'
            }}
          </p>
        </div>

        <button
          class="form-button"
          data-testid="new-engagement-button"
          @click="goToNewEngagement"
        >
          New Engagement
        </button>
      </div>

      <p v-if="!engagements.length" class="empty-state">
        No engagements yet. Create one to get started.
      </p>

      <div v-else class="engagement-list">
        <article
          v-for="eng in engagements"
          :key="eng.id"
          class="ops-card engagement-row"
          @click="goToEngagement(eng.id)"
        >
          <div>
            <p class="eyebrow">{{ eng.status }}</p>
            <h3>{{ eng.title }}</h3>
            <p>{{ eng.contractor_name }} — {{ eng.role }}</p>
          </div>

          <div class="engagement-meta">
            <strong>${{ (eng.amount_cents / 100).toFixed(2) }}</strong>
            <small>Due: {{ eng.due_date || '—' }}</small>
          </div>
        </article>
      </div>
    </section>
  </DashboardShell>
</template>

<style scoped>
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
  cursor: pointer;
}

.form-button {
  background: linear-gradient(90deg, #60a5fa, #34d399);
  border: 0;
  border-radius: 12px;
  color: #020617;
  cursor: pointer;
  font-weight: 900;
  padding: 11px 16px;
}

.form-error,
.empty-state {
  border-radius: 14px;
  padding: 16px;
}

.form-error {
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
}

.empty-state {
  border: 1px dashed rgba(45, 212, 191, 0.28);
  color: #cbd5e1;
}
</style>
