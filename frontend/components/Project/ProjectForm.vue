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

    <button class="form-button" type="submit">Save Project</button>
  </form>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref, watch } from 'vue';
import { useApi } from '~/composables/useApi';
import { useClients } from '~/composables/useClients';

const emit = defineEmits<{
  submit: [payload: any];
}>();

const props = defineProps<{
  modelValue?: any;
}>();

const { get } = useApi();
const { getOrganizationClients } = useClients();

const clients = ref<any[]>([]);

const form = reactive({
  client_id: props.modelValue?.client_id ?? 0,
  name: props.modelValue?.name ?? '',
  start_date: props.modelValue?.start_date ?? '',
  end_date: props.modelValue?.end_date ?? '',
  description: props.modelValue?.description ?? '',
});

async function loadClients() {
  const organization = await get('/api/me/organization');
  clients.value = await getOrganizationClients(organization.id);

  if (!form.client_id && clients.value.length) {
    form.client_id = clients.value[0].id;
  }
}

function submit() {
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

onMounted(loadClients);
</script>

<style scoped>
@import '../../assets/css/forms.css';

.project::before {
  background: linear-gradient(135deg, #22c55e, #3b82f6);
}
</style>
