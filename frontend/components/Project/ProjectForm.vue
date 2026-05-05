<template>
  <form class="form-shell form-card project" @submit.prevent="submit">
    <h3 class="form-title">Create Project</h3>

    <div class="grid form-group">
      <label class="form-label"> Client ID: </label>

      <input
        v-model.number="form.client_id"
        type="number"
        placeholder="Client ID"
        class="form-input"
      />
      <label class="form-label"> Project Name: </label>
      <input
        v-model="form.name"
        placeholder="Project Name"
        class="form-input"
      />
      <label class="form-label"> Start Date: </label>

      <input
        v-model="form.start_date"
        placeholder="Start Date"
        class="form-input"
      />
      <label class="form-label"> End Date: </label>

      <input
        v-model="form.end_date"
        placeholder="End Date"
        class="form-input"
      />

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
@import '../../assets/css/forms.css';
.project {
  /* background: #f0fdf4; */
}
.project::before {
  background: linear-gradient(135deg, #22c55e, #3b82f6);
}
</style>
