<template>
  <form class="editor" @submit.prevent="submit">
    <label>
      User Type
      <select v-model="selectedType">
        <option value="admin">Admin</option>
        <option value="consultant">Consultant</option>
        <option value="client">Client</option>
      </select>
    </label>

    <button type="submit">Save</button>
  </form>
</template>

<script setup lang="ts">
import { watch } from 'fs';
import { ref } from 'process';

const props = defineProps<{
  userType: string;
}>();

const emit = defineEmits<{
  (e: 'submit', payload: { user_type: string }): void;
}>();

const selectedType = ref(props.userType);

watch(
  () => props.userType,
  (value) => {
    selectedType.value = value;
  },
);

const submit = () => {
  emit('submit', { user_type: selectedType.value });
};
</script>

<style scoped>
.editor {
  display: flex;
  gap: 0.75rem;
  align-items: end;
}

label {
  display: grid;
  gap: 0.35rem;
  color: #9fb3c8;
}

select {
  padding: 0.6rem;
  border-radius: 0.65rem;
  border: 1px solid rgba(148, 163, 184, 0.35);
  background: #0f1d2b;
  color: white;
}

button {
  border: 0;
  border-radius: 0.7rem;
  padding: 0.65rem 0.9rem;
  font-weight: 700;
  background: linear-gradient(135deg, #60a5fa, #34d399);
}
</style>
