<template>
  <form class="form-card project" @submit.prevent="submit">
    <h3>Create Project</h3>

    <div class="grid">
      <input
        v-model.number="form.client_id"
        type="number"
        placeholder="Client ID"
      />
      <input v-model="form.name" placeholder="Project Name" />

      <input v-model="form.start_date" placeholder="Start Date" />
      <input v-model="form.end_date" placeholder="End Date" />

      <textarea v-model="form.description" placeholder="Description" />
    </div>

    <button type="submit">Save Project</button>
  </form>
</template>

<script setup lang="ts">
import { reactive } from 'vue';

const form = reactive({
  client_id: 0,
  name: '',
  start_date: '',
  end_date: '',
  description: '',
});

const submit = () => console.log('Project:', form);
</script>

<style scoped>
@import '../form.css';
.project {
  background: #f0fdf4;
}
.project::before {
  background: linear-gradient(135deg, #22c55e, #3b82f6);
}
</style>
