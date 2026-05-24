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
  role: props.modelValue?.role ?? 'full_stack_developer',
  title: props.modelValue?.title ?? '',
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
    if (!form.project_id && projects?.length) {
      form.project_id = projects[0].id;
    }
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
  <form
    class="form-shell rounded-2xl border border-cyan-400/70 bg-slate-950/95 p-6 shadow-[0_0_40px_rgba(34,211,238,0.12)] space-y-5"
    @submit.prevent="submit"
  >
    <div>
      <p class="form-title text-xs uppercase tracking-[0.24em] text-cyan-300">
        Software Engagement
      </p>
      <h2 class="form-subtitle mt-2 text-2xl font-bold text-white">
        Create contractor workspace
      </h2>
      <p class="form-label mt-2 text-sm text-slate-300">
        Define the developer, scope, deliverables, repo, and payment terms.
      </p>
    </div>
    <div class="form-group space-y-2">
      <label class="form-label text-sm font-medium text-slate-100">
        Project
      </label>

      <select
        v-model.number="form.project_id"
        class="form-input w-full rounded-xl border border-slate-600 bg-slate-900 px-4 py-3 text-white focus:border-cyan-300 focus:outline-none"
        required
      >
        <option disabled :value="0">
          {{ loading ? 'Loading projects...' : 'Select project' }}
        </option>

        <option
          v-for="project in props.projects"
          :key="project.id"
          :value="project.id"
        >
          {{ project.name }}
        </option>
      </select>
    </div>
    <div class="form-group grid gap-4 md:grid-cols-2">
      <div class="space-y-2">
        <label class="form-label text-sm font-medium text-slate-100"
          >Contractor Name</label
        >
        <input
          v-model="form.contractor_name"
          class="form-input w-full rounded-xl border border-slate-600 bg-slate-900 px-4 py-3 text-white placeholder:text-slate-500 focus:border-cyan-300 focus:outline-none"
          placeholder="Jane Developer"
          required
        />
      </div>

      <div class="space-y-2">
        <label class="form-label text-sm font-medium text-slate-100"
          >Contractor Email</label
        >
        <input
          v-model="form.contractor_email"
          type="email"
          class="form-input w-full rounded-xl border border-slate-600 bg-slate-900 px-4 py-3 text-white placeholder:text-slate-500 focus:border-cyan-300 focus:outline-none"
          placeholder="jane@example.com"
          required
        />
      </div>
    </div>

    <div class="form-group grid gap-4 md:grid-cols-2">
      <div class="space-y-2">
        <label class="form-label text-sm font-medium text-slate-100">
          Role:
        </label>
        <select
          v-model="form.role"
          class="form-input w-full rounded-xl border border-slate-600 bg-slate-900 px-4 py-3 text-white focus:border-cyan-300 focus:outline-none"
        >
          <option value="full_stack_developer">Full-stack Developer</option>
          <option value="frontend_developer">Frontend Developer</option>
          <option value="backend_developer">Backend Developer</option>
          <option value="designer">Designer</option>
          <option value="devops_engineer">DevOps Engineer</option>
        </select>
      </div>

      <div class="space-y-2">
        <label class="form-label text-sm font-medium text-slate-100"
          >Engagement Title</label
        >
        <input
          v-model="form.title"
          class="form-input w-full rounded-xl border border-slate-600 bg-slate-900 px-4 py-3 text-white placeholder:text-slate-500 focus:border-cyan-300 focus:outline-none"
          placeholder="Build Client Portal MVP"
          required
        />
      </div>
    </div>

    <div class="form-group space-y-2">
      <label class="form-label text-sm font-medium text-slate-100"
        >Scope of Work</label
      >
      <textarea
        v-model="form.scope_of_work"
        class="form-input w-full rounded-xl border border-slate-600 bg-slate-900 px-4 py-3 text-white placeholder:text-slate-500 focus:border-cyan-300 focus:outline-none"
        rows="5"
        placeholder="Build authentication, organization switcher, engagement tracker, and billing flow."
        required
      />
    </div>

    <div class="form-group space-y-2">
      <label class="form-label text-sm font-medium text-slate-100"
        >Deliverables</label
      >
      <textarea
        v-model="form.deliverables"
        class="form-input w-full rounded-xl border border-slate-600 bg-slate-900 px-4 py-3 text-white placeholder:text-slate-500 focus:border-cyan-300 focus:outline-none"
        rows="3"
        placeholder="Rust API, Nuxt frontend, deployment notes, passing tests."
      />
    </div>

    <div class="form-group grid gap-4 md:grid-cols-3">
      <div class="space-y-2 md:col-span-1">
        <label class="form-label text-sm font-medium text-slate-100"
          >Repo URL</label
        >
        <input
          v-model="form.repo_url"
          class="form-input w-full rounded-xl border border-slate-600 bg-slate-900 px-4 py-3 text-white placeholder:text-slate-500 focus:border-cyan-300 focus:outline-none"
          placeholder="https://github.com/..."
        />
      </div>

      <div class="space-y-2">
        <label class="form-label text-sm font-medium text-slate-100"
          >Amount in cents</label
        >
        <input
          v-model="form.amount_cents"
          type="number"
          class="form-input w-full rounded-xl border border-slate-600 bg-slate-900 px-4 py-3 text-white placeholder:text-slate-500 focus:border-cyan-300 focus:outline-none"
          placeholder="200000"
          required
        />
      </div>

      <div class="space-y-2">
        <label class="form-label text-sm font-medium text-slate-100"
          >Due Date</label
        >
        <input
          v-model="form.due_date"
          type="date"
          class="form-input w-full rounded-xl border border-slate-600 bg-slate-900 px-4 py-3 text-white focus:border-cyan-300 focus:outline-none"
        />
      </div>
    </div>

    <p
      v-if="error"
      class="rounded-xl border border-red-400/40 bg-red-950/40 p-3 text-sm text-red-200"
    >
      {{ error }}
    </p>

    <button
      class="form-button w-full rounded-xl bg-gradient-to-r from-sky-400 to-emerald-400 px-5 py-3 text-sm font-bold text-slate-950 shadow-lg transition hover:scale-[1.01] disabled:cursor-not-allowed disabled:opacity-60"
      :disabled="loading"
    >
      {{ loading ? 'Creating engagement...' : 'Create Software Engagement' }}
    </button>
  </form>
</template>
