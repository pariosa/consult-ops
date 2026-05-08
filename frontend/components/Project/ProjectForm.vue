<template>
  <form class="form-shell form-card project" @submit.prevent="submit">
    <h3 class="form-title">Create Project</h3>

    <div class="grid form-group">
      <label class="form-label">Client</label>
      <select v-model.number="form.client_id" class="form-input" required>
        <option disabled :value="0">Select client</option>

        <option v-for="client in clients" :key="client.id" :value="client.id">
          {{ client.name
          }}{{ client.company_name ? ` — ${client.company_name}` : '' }}
        </option>
      </select>

      <label class="form-label">Project Name</label>
      <input
        v-model="form.name"
        placeholder="Project Name"
        class="form-input"
        required
      />

      <label class="form-label">Start Date</label>
      <input v-model="form.start_date" type="date" class="form-input" />

      <label class="form-label">End Date</label>
      <input v-model="form.end_date" type="date" class="form-input" />

      <textarea
        v-model="form.description"
        placeholder="Description"
        class="form-textarea"
      />
    </div>

    <p v-if="error" class="form-error">
      {{ error }}
    </p>

    <button class="form-button" type="submit">Save Project</button>
  </form>
</template>

<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue';

const props = defineProps<{
  clients?: any[];
  modelValue?: any;
}>();

const emit = defineEmits<{
  submit: [payload: any];
}>();

const error = ref('');

const clients = computed(() => props.clients ?? []);

const form = reactive({
  client_id: props.modelValue?.client_id ?? props.clients?.[0]?.id ?? 0,
  name: props.modelValue?.name ?? '',
  start_date: props.modelValue?.start_date ?? '',
  end_date: props.modelValue?.end_date ?? '',
  description: props.modelValue?.description ?? '',
});

function submit() {
  error.value = '';

  if (!form.client_id) {
    error.value = 'Select a client before creating a project.';
    return;
  }

  emit('submit', { ...form });
}

watch(
  () => props.modelValue,
  (value) => {
    if (!value) return;
    Object.assign(form, value);
  },
  { deep: true },
);

watch(
  () => props.clients,
  (value) => {
    if (!form.client_id && value?.length) {
      form.client_id = value[0].id;
    }
  },
  { deep: true, immediate: true },
);
</script>

<style scoped>
@import '../../assets/css/forms.css';

.project::before {
  background: linear-gradient(135deg, #22c55e, #3b82f6);
}
</style>
