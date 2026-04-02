<template>
  <div class="card">
    <h3>{{ project.title }}</h3>
    <p>{{ project.description }}</p>
    <p>Status: {{ project.status }}</p>
    <p>{{ project.start_date }} → {{ project.end_date }}</p>
  </div>
</template>

<script setup lang="ts">
defineProps<{ project: any }>()
</script>

<style scoped>
.card { border: 1px solid #ccc; border-radius: 8px; padding: 1rem; margin: 0.5rem 0; background:#f5f5f5; }
</style>