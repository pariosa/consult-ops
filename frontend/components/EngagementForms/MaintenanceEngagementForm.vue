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
  role: props.modelValue?.role ?? 'maintenance_contractor',
  title: props.modelValue?.title ?? 'Maintenance Retainer',
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
      <p class="form-title">Maintenance / Retainer Engagement</p>
      <h2>Recurring support workspace</h2>
      <p>
        Define support coverage, response expectations, recurring deliverables,
        and maintenance logs.
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
        <label>Contractor Name</label>
        <input
          v-model="form.contractor_name"
          class="form-input"
          placeholder="Taylor Support"
          required
        />
      </div>

      <div>
        <label>Contractor Email</label>
        <input
          v-model="form.contractor_email"
          type="email"
          class="form-input"
          placeholder="support@example.com"
          required
        />
      </div>
    </div>

    <div class="form-grid">
      <div>
        <label>Support Role</label>
        <select v-model="form.role" class="form-input">
          <option value="maintenance_contractor">Maintenance Contractor</option>
          <option value="support_engineer">Support Engineer</option>
          <option value="systems_administrator">Systems Administrator</option>
          <option value="operations_support">Operations Support</option>
          <option value="site_reliability_support">
            Site Reliability Support
          </option>
        </select>
      </div>

      <div>
        <label>Engagement Title</label>
        <input
          v-model="form.title"
          class="form-input"
          placeholder="Monthly Support Retainer"
          required
        />
      </div>
    </div>

    <label>Support Coverage / Scope</label>
    <textarea
      v-model="form.scope_of_work"
      class="form-input"
      rows="5"
      placeholder="Provide recurring support, monitoring, bug fixes, operational maintenance, and response to approved support requests."
      required
    />

    <label>Recurring Deliverables</label>
    <textarea
      v-model="form.deliverables"
      class="form-input"
      rows="3"
      placeholder="Monthly summary, resolved tickets, maintenance log, uptime notes, recommendations."
    />

    <label>Support Board / System URL</label>
    <input
      v-model="form.repo_url"
      class="form-input"
      placeholder="Ticket board, monitoring dashboard, repo, or support workspace URL"
    />

    <div class="form-grid">
      <div>
        <label>Retainer Amount in cents</label>
        <input
          v-model="form.amount_cents"
          type="number"
          class="form-input"
          required
        />
      </div>

      <div>
        <label>Review / Renewal Date</label>
        <input v-model="form.due_date" type="date" class="form-input" />
      </div>
    </div>

    <p v-if="error" class="form-error">{{ error }}</p>

    <button class="form-button" :disabled="loading">
      {{ loading ? 'Creating engagement...' : 'Create Maintenance Engagement' }}
    </button>
  </form>
</template>
