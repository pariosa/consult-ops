<script setup lang="ts">
import { reactive, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useEngagements } from '~/composables/useEngagements';

const route = useRoute();
const router = useRouter();

const projectId = Number(route.query.projectId || route.params.projectId || 1);

const { createEngagement } = useEngagements();

const form = reactive({
  contractor_name: '',
  contractor_email: '',
  role: 'full_stack_developer',
  title: '',
  scope_of_work: '',
  deliverables: '',
  repo_url: '',
  amount_cents: 0,
  due_date: '',
});

const loading = ref(false);
const error = ref('');
const success = ref('');

async function submit() {
  loading.value = true;
  error.value = '';
  success.value = '';

  try {
    const engagement = await createEngagement(projectId, form);

    success.value = 'Engagement created successfully';

    // redirect to engagement page
    setTimeout(() => {
      router.push(`/engagements/${engagement.id}`);
    }, 800);
  } catch (err: any) {
    error.value = err?.message || 'Failed to create engagement';
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <main class="p-6 max-w-5xl mx-auto">
    <div class="form-shell">
      <div>
        <p class="form-eyebrow">Engagement</p>
        <h1 class="form-title">New Software Engagement</h1>
        <p class="form-subtitle">
          Create a contractor agreement and define the work scope.
        </p>
      </div>

      <form @submit.prevent="submit" class="space-y-5">
        <div class="form-grid two">
          <div class="form-group">
            <label class="form-label">Contractor Name</label>
            <input v-model="form.contractor_name" class="form-input" required />
          </div>

          <div class="form-group">
            <label class="form-label">Contractor Email</label>
            <input
              v-model="form.contractor_email"
              type="email"
              class="form-input"
              required
            />
          </div>
        </div>

        <div class="form-grid two">
          <div class="form-group">
            <label class="form-label">Role</label>
            <select v-model="form.role" class="form-select">
              <option value="full_stack_developer">Full-stack Developer</option>
              <option value="frontend_developer">Frontend Developer</option>
              <option value="backend_developer">Backend Developer</option>
            </select>
          </div>

          <div class="form-group">
            <label class="form-label">Title</label>
            <input v-model="form.title" class="form-input" required />
          </div>
        </div>

        <div class="form-group">
          <label class="form-label">Scope of Work</label>
          <textarea
            v-model="form.scope_of_work"
            class="form-textarea"
            required
          />
        </div>

        <div class="form-group">
          <label class="form-label">Deliverables</label>
          <textarea v-model="form.deliverables" class="form-textarea" />
        </div>

        <div class="form-grid three">
          <div class="form-group">
            <label class="form-label">Repo URL</label>
            <input v-model="form.repo_url" class="form-input" />
          </div>

          <div class="form-group">
            <label class="form-label">Amount (cents)</label>
            <input
              v-model="form.amount_cents"
              type="number"
              class="form-input"
              required
            />
          </div>

          <div class="form-group">
            <label class="form-label">Due Date</label>
            <input v-model="form.due_date" type="date" class="form-input" />
          </div>
        </div>

        <div v-if="error" class="form-error">
          {{ error }}
        </div>

        <div v-if="success" class="form-success">
          {{ success }}
        </div>

        <button class="form-button" :disabled="loading">
          {{ loading ? 'Creating...' : 'Create Engagement' }}
        </button>
      </form>
    </div>
  </main>
</template>
