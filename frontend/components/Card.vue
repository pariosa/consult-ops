<template>
  <div class="card">
    <h3>{{ title }}</h3>
    <p>{{ subtitle }}</p>
  </div>
</template>

<script setup lang="ts">
defineProps<{ title: string, subtitle: string }>()
</script>

<style scoped>
.card { border: 1px solid #ccc; border-radius: 8px; padding: 1rem; margin: 1rem 0; }
</style>