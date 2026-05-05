<script setup lang="ts">
import { reactive, ref } from 'vue';
import { useEngagementMilestones } from '../../composables/useEngagementMilestones';

const props = defineProps<{
  engagementId: number;
  mockSubmit?: boolean;
}>();

const emit = defineEmits<{
  created: [milestone: any];
}>();

const { createMilestone } = useEngagementMilestones();

const form = reactive({
  engagement_id: props.engagementId,
  title: '',
  description: '',
  amount_cents: 0,
  due_date: '',
});

const loading = ref(false);
const error = ref('');

async function submit() {
  loading.value = true;
  error.value = '';

  try {
    if (props.mockSubmit) {
      const milestone = {
        id: Date.now(),
        engagement_id: props.engagementId,
        title: form.title,
        description: form.description,
        amount_cents: Number(form.amount_cents),
        due_date: form.due_date,
        status: 'pending',
        created_at: new Date().toISOString(),
      };

      emit('created', milestone);
      resetForm();
      return;
    }

    const milestone = await createMilestone(props.engagementId, {
      ...form,
      engagement_id: props.engagementId,
      amount_cents: Number(form.amount_cents),
    });

    emit('created', milestone);
    resetForm();
  } catch (err: any) {
    error.value = err?.message || 'Could not create milestone';
  } finally {
    loading.value = false;
  }
}

function resetForm() {
  form.title = '';
  form.description = '';
  form.amount_cents = 0;
  form.due_date = '';
}
</script>

<template>
  <form
    class="form-shell rounded-2xl border border-cyan-400/70 bg-slate-950/95 p-6 shadow-[0_0_40px_rgba(34,211,238,0.12)]"
    @submit.prevent="submit"
  >
    <div>
      <p class="form-title text-xs uppercase tracking-[0.24em] text-cyan-300">
        Milestone
      </p>
      <h2 class="form-subtitle text-2xl font-bold text-white">
        Add project milestone
      </h2>
      <p class="form-label text-sm text-slate-300">
        Break the software engagement into reviewable, payable chunks.
      </p>
    </div>

    <div class="form-group">
      <label
        for="milestone-title"
        class="form-label text-sm font-medium text-slate-100"
      >
        Title
      </label>
      <input
        id="milestone-title"
        v-model="form.title"
        data-testid="title"
        class="form-input w-full rounded-xl border border-slate-600 bg-slate-900 text-white placeholder:text-slate-500 focus:border-cyan-300 focus:outline-none"
        placeholder="Stripe billing integration"
        required
      />
    </div>

    <div class="form-group">
      <label
        for="milestone-description"
        class="form-label text-sm font-medium text-slate-100"
      >
        Description
      </label>
      <textarea
        id="milestone-description"
        v-model="form.description"
        data-testid="description"
        class="form-input w-full rounded-xl border border-slate-600 bg-slate-900 text-white placeholder:text-slate-500 focus:border-cyan-300 focus:outline-none"
        rows="4"
        placeholder="Implement activation checkout, webhook handling, and billing status updates."
      />
    </div>

    <div class="form-group">
      <div class="">
        <label
          for="milestone-amount"
          class="form-label text-sm font-medium text-slate-100"
        >
          Amount in cents
        </label>
        <input
          id="milestone-amount"
          v-model="form.amount_cents"
          data-testid="amount"
          type="number"
          class="form-input w-full rounded-xl border border-slate-600 bg-slate-900 text-white placeholder:text-slate-500 focus:border-cyan-300 focus:outline-none"
          placeholder="50000"
          required
        />
      </div>

      <div class="form-group">
        <label
          for="milestone-due-date"
          class="form-label text-sm font-medium text-slate-100"
        >
          Due Date
        </label>
        <input
          id="milestone-due-date"
          v-model="form.due_date"
          data-testid="due_date"
          type="date"
          class="form-input w-full rounded-xl border border-slate-600 bg-slate-900 text-white focus:border-cyan-300 focus:outline-none"
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
      {{ loading ? 'Adding milestone...' : 'Add Milestone' }}
    </button>
  </form>
</template>
