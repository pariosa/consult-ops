<template>
  <section class="panel">
    <header class="panel-header">
      <div>
        <h2>Users</h2>
        <p>Manage platform users and account types.</p>
      </div>
    </header>

    <table v-if="users.length">
      <thead>
        <tr>
          <th>Email</th>
          <th>Name</th>
          <th>Type</th>
          <th>Updated</th>
          <th>Profile</th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="user in users" :key="user.id">
          <td>{{ user.email }}</td>
          <td>{{ user.name || '—' }}</td>
          <td>
            <span class="badge">{{ user.user_type }}</span>
          </td>
          <td>{{ user.updated_at || '—' }}</td>
          <td>
            <NuxtLink :to="`/admin/users/${user.id}`">Edit</NuxtLink>
          </td>
        </tr>
      </tbody>
    </table>

    <p v-else class="empty">No users found.</p>
  </section>
</template>

<script setup lang="ts">
defineProps<{
  users: any[];
}>();
</script>

<style scoped>
.panel {
  padding: 1.25rem;
  border: 1px solid rgba(80, 210, 170, 0.25);
  border-radius: 1.25rem;
  background: rgba(8, 19, 31, 0.82);
}

.panel-header {
  margin-bottom: 1rem;
}

h2 {
  margin: 0;
}

p {
  color: #9fb3c8;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th,
td {
  padding: 0.85rem;
  border-bottom: 1px solid rgba(148, 163, 184, 0.18);
  text-align: left;
}

th {
  color: #9fb3c8;
}

.badge {
  padding: 0.25rem 0.6rem;
  border-radius: 999px;
  background: rgba(96, 165, 250, 0.16);
  color: #7dd3fc;
}

a {
  color: #6ee7b7;
}

.empty {
  padding: 1rem 0;
}
</style>
