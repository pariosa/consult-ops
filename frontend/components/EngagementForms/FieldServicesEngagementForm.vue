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
  role: props.modelValue?.role ?? 'field_specialist',
  title: props.modelValue?.title ?? 'Field Services Engagement',
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
      <p class="form-title">Field Services Engagement</p>
      <h2>On-site / operational service workspace</h2>
      <p>
        Define the field specialist, service location, task checklist, and
        completion evidence.
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
        <label>Field Specialist Name</label>
        <input
          v-model="form.contractor_name"
          class="form-input"
          placeholder="Alex Fielding"
          required
        />
      </div>

      <div>
        <label>Field Specialist Email</label>
        <input
          v-model="form.contractor_email"
          type="email"
          class="form-input"
          placeholder="alex@example.com"
          required
        />
      </div>
    </div>

    <div class="form-grid">
      <div>
        <label>Service Role</label>
        <select v-model="form.role" class="form-input">
          <option value="field_specialist">Field Specialist</option>
          <option value="inspection_contractor">Inspection Contractor</option>
          <option value="installation_technician">
            Installation Technician
          </option>
          <option value="maintenance_technician">Maintenance Technician</option>
          <option value="site_operator">Site Operator</option>
        </select>
      </div>

      <div>
        <label>Engagement Title</label>
        <input
          v-model="form.title"
          class="form-input"
          placeholder="Site Inspection and Service Visit"
          required
        />
      </div>
    </div>

    <label>Field Scope / Site Instructions</label>
    <textarea
      v-model="form.scope_of_work"
      class="form-input"
      rows="5"
      placeholder="Visit site, inspect equipment, document findings, complete checklist, and report exceptions."
      required
    />

    <label>Completion Evidence / Deliverables</label>
    <textarea
      v-model="form.deliverables"
      class="form-input"
      rows="3"
      placeholder="Completed checklist, photos, inspection report, service notes, client sign-off."
    />

    <label>Site / Work Order URL</label>
    <input
      v-model="form.repo_url"
      class="form-input"
      placeholder="Maps link, work order, shared folder, or ticket URL"
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
        <label>Service Due Date</label>
        <input v-model="form.due_date" type="date" class="form-input" />
      </div>
    </div>

    <p v-if="error" class="form-error">{{ error }}</p>

    <button class="form-button" :disabled="loading">
      {{
        loading ? 'Creating engagement...' : 'Create Field Services Engagement'
      }}
    </button>
  </form>
</template>
