<script setup lang="ts">
import { reactive, ref, watch } from 'vue';

const props = defineProps<{
  projects?: any[];
  modelValue?: any;
  loading?: boolean;
}>();

const emit = defineEmits<{
  submit: [payload: any];
}>();

const error = ref('');

const form = reactive({
  project_id: props.modelValue?.project_id ?? props.projects?.[0]?.id ?? 0,
  contractor_name: props.modelValue?.contractor_name ?? '',
  contractor_email: props.modelValue?.contractor_email ?? '',
  role: props.modelValue?.role ?? 'strategy_consultant',
  title: props.modelValue?.title ?? 'Consulting Engagement',
  scope_of_work: props.modelValue?.scope_of_work ?? '',
  deliverables: props.modelValue?.deliverables ?? '',
  repo_url: props.modelValue?.repo_url ?? '',
  amount_cents: props.modelValue?.amount_cents ?? 0,
  currency: props.modelValue?.currency ?? 'usd',
  due_date: props.modelValue?.due_date ?? '',
});

watch(
  () => props.projects,
  (projects) => {
    if (!form.project_id && projects?.length) form.project_id = projects[0].id;
  },
  { immediate: true, deep: true },
);

function submit() {
  error.value = '';

  if (!form.project_id) {
    error.value = 'Select a project before creating an engagement.';
    return;
  }

  emit('submit', {
    ...form,
    amount_cents: Number(form.amount_cents),
    due_date: form.due_date || null,
  });
}
</script>

<template>
  <form class="form-shell" @submit.prevent="submit">
    <div>
      <p class="form-title">Consulting Engagement</p>
      <h2>Advisory / implementation workspace</h2>
      <p>
        Define the consultant, objectives, advisory scope, and final
        recommendations.
      </p>
    </div>

    <label>Project</label>
    <select v-model.number="form.project_id" class="form-input" required>
      <option disabled :value="0">Select project</option>
      <option
        v-for="project in props.projects"
        :key="project.id"
        :value="project.id"
      >
        {{ project.name }}
      </option>
    </select>

    <div class="form-grid">
      <div>
        <label>Consultant Name</label>
        <input
          v-model="form.contractor_name"
          class="form-input"
          placeholder="Morgan Advisor"
          required
        />
      </div>

      <div>
        <label>Consultant Email</label>
        <input
          v-model="form.contractor_email"
          type="email"
          class="form-input"
          placeholder="morgan@example.com"
          required
        />
      </div>
    </div>

    <div class="form-grid">
      <div>
        <label>Consulting Role</label>
        <select v-model="form.role" class="form-input">
          <option value="strategy_consultant">Strategy Consultant</option>
          <option value="operations_consultant">Operations Consultant</option>
          <option value="technical_consultant">Technical Consultant</option>
          <option value="implementation_consultant">
            Implementation Consultant
          </option>
          <option value="financial_consultant">Financial Consultant</option>
        </select>
      </div>

      <div>
        <label>Engagement Title</label>
        <input
          v-model="form.title"
          class="form-input"
          placeholder="Operations Improvement Assessment"
          required
        />
      </div>
    </div>

    <label>Consulting Objectives / Scope</label>
    <textarea
      v-model="form.scope_of_work"
      class="form-input"
      rows="5"
      placeholder="Assess current process, identify operational gaps, recommend improvements, and support implementation planning."
      required
    />

    <label>Deliverables</label>
    <textarea
      v-model="form.deliverables"
      class="form-input"
      rows="3"
      placeholder="Discovery notes, recommendations report, implementation roadmap, stakeholder presentation."
    />

    <label>Reference / Workspace URL</label>
    <input
      v-model="form.repo_url"
      class="form-input"
      placeholder="Shared folder, Notion, Drive, or workspace URL"
    />

    <div class="form-grid">
      <div>
        <label>Amount in cents</label>
        <input
          v-model="form.amount_cents"
          type="number"
          class="form-input"
          required
        />
      </div>

      <div>
        <label>Due Date</label>
        <input v-model="form.due_date" type="date" class="form-input" />
      </div>
    </div>

    <p v-if="error" class="form-error">{{ error }}</p>

    <button class="form-button" :disabled="loading">
      {{ loading ? 'Creating engagement...' : 'Create Consulting Engagement' }}
    </button>
  </form>
</template>
