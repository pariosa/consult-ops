<script setup lang="ts">
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
    error.value = err?.message || 'Failed to load project';
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
  <main class="p-6 max-w-5xl mx-auto space-y-6">
    <!-- HEADER -->
    <section class="form-shell">
      <div>
        <p class="form-eyebrow">Project</p>
        <h1 class="form-title">
          {{ project?.name || 'Loading project...' }}
        </h1>
        <p class="form-subtitle">
          Manage engagements, contractors, and delivery for this project.
        </p>
      </div>

      <div class="form-actions">
        <button class="form-button" @click="goToNewEngagement">
          New Engagement
        </button>
      </div>
    </section>

    <!-- STATES -->
    <section v-if="loading" class="text-slate-400">Loading...</section>

    <section v-else-if="error" class="form-error">
      {{ error }}
    </section>

    <!-- ENGAGEMENT LIST -->
    <section v-else class="space-y-4">
      <h2 class="text-white text-xl font-bold">Engagements</h2>

      <div v-if="!engagements.length" class="text-slate-400">
        No engagements yet. Create one to get started.
      </div>

      <div
        v-for="eng in engagements"
        :key="eng.id"
        class="rounded-xl border border-slate-600 bg-slate-900 p-4 hover:border-cyan-400 cursor-pointer transition"
        @click="goToEngagement(eng.id)"
      >
        <div class="flex justify-between items-start gap-4">
          <div>
            <h3 class="text-white font-bold">
              {{ eng.title }}
            </h3>

            <p class="text-slate-400 text-sm mt-1">
              {{ eng.contractor_name }} • {{ eng.role }}
            </p>

            <p class="text-slate-500 text-xs mt-2">Status: {{ eng.status }}</p>
          </div>

          <div class="text-right">
            <p class="text-white font-semibold">
              ${{ (eng.amount_cents / 100).toFixed(2) }}
            </p>

            <p class="text-slate-500 text-xs">Due: {{ eng.due_date || '—' }}</p>
          </div>
        </div>
      </div>
    </section>
  </main>
</template>
