<!-- components/engagements/RentalEngagementForm.vue -->
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
  role: props.modelValue?.role ?? 'rental_provider',
  title: props.modelValue?.title ?? 'Rental Engagement',
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
    error.value = 'Select a project before creating a rental engagement.';
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
      <p class="form-title">Rental Engagement</p>
      <h2>Equipment / venue rental workspace</h2>
      <p>
        Define the rental provider, asset or venue, rental period, usage terms,
        return expectations, and payment terms.
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
        <label>Provider / Owner Name</label>
        <input
          v-model="form.contractor_name"
          class="form-input"
          placeholder="Harbor Venue Co."
          required
        />
      </div>

      <div>
        <label>Provider Email</label>
        <input
          v-model="form.contractor_email"
          type="email"
          class="form-input"
          placeholder="rentals@example.com"
          required
        />
      </div>
    </div>

    <div class="form-grid">
      <div>
        <label>Rental Type</label>
        <select v-model="form.role" class="form-input">
          <option value="rental_provider">Rental Provider</option>
          <option value="equipment_owner">Equipment Owner</option>
          <option value="venue_owner">Venue Owner</option>
          <option value="event_space_provider">Event Space Provider</option>
          <option value="vehicle_equipment_provider">
            Vehicle / Equipment Provider
          </option>
        </select>
      </div>

      <div>
        <label>Engagement Title</label>
        <input
          v-model="form.title"
          class="form-input"
          placeholder="Camera Equipment Rental"
          required
        />
      </div>
    </div>

    <label>Rental Scope / Usage Terms</label>
    <textarea
      v-model="form.scope_of_work"
      class="form-input"
      rows="5"
      placeholder="Rent listed equipment or venue for the agreed period. Include permitted use, access instructions, pickup/dropoff or check-in/check-out details, and any restrictions."
      required
    />

    <label>Included Assets / Return Conditions</label>
    <textarea
      v-model="form.deliverables"
      class="form-input"
      rows="3"
      placeholder="Itemized equipment list, venue areas included, keys/access codes, condition report, return checklist, cleaning or damage expectations."
    />

    <label>Inventory / Booking / Location URL</label>
    <input
      v-model="form.repo_url"
      class="form-input"
      placeholder="Inventory sheet, booking page, map link, shared folder, or venue listing URL"
    />

    <div class="form-grid">
      <div>
        <label>Rental Amount in cents</label>
        <input
          v-model="form.amount_cents"
          type="number"
          class="form-input"
          placeholder="75000"
          required
        />
      </div>

      <div>
        <label>Return / End Date</label>
        <input v-model="form.due_date" type="date" class="form-input" />
      </div>
    </div>

    <p v-if="error" class="form-error">{{ error }}</p>

    <button class="form-button" :disabled="loading">
      {{ loading ? 'Creating engagement...' : 'Create Rental Engagement' }}
    </button>
  </form>
</template>
