<!-- components/UserCreateForm.vue -->
<template>
  <form class="panel" @submit.prevent="submit">
    <h2>Create User</h2>

    <input v-model="name" placeholder="Name" />
    <input v-model="email" type="email" placeholder="Email" />
    <input
      v-model="password"
      type="password"
      placeholder="Temporary password"
    />

    <select v-model="userType">
      <option value="consultant">Consultant</option>
      <option value="client">Client</option>
      <option value="admin">Admin</option>
    </select>

    <button>Create user</button>
  </form>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const emit = defineEmits<{
  (
    e: 'submit',
    payload: {
      name: string;
      email: string;
      password: string;
      user_type: string;
    },
  ): void;
}>();

const name = ref('');
const email = ref('');
const password = ref('');
const userType = ref('consultant');

const submit = () => {
  emit('submit', {
    name: name.value,
    email: email.value,
    password: password.value,
    user_type: userType.value,
  });

  name.value = '';
  email.value = '';
  password.value = '';
  userType.value = 'consultant';
};
</script>

<style scoped>
.panel {
  display: grid;
  gap: 0.85rem;
  max-width: 520px;
  padding: 1.25rem;
  margin: 1.5rem 0;
  border: 1px solid rgba(80, 210, 170, 0.25);
  border-radius: 1.25rem;
  background: rgba(8, 19, 31, 0.82);
}

input,
select {
  padding: 0.75rem;
  border-radius: 0.75rem;
  border: 1px solid rgba(148, 163, 184, 0.35);
  background: #0f1d2b;
  color: white;
}

button {
  border: 0;
  border-radius: 0.85rem;
  padding: 0.85rem;
  font-weight: 700;
  background: linear-gradient(135deg, #60a5fa, #34d399);
}
</style>
