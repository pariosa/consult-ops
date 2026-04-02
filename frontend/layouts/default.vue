<template>
  <div>
    <header class="header">
      <h1>Consultant Ops</h1>
      <button @click="toggleTheme">Toggle Theme</button>
    </header>
    <main>
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts">
import { toggleTheme } from '~/composables/useTheme'
</script>

<style scoped>
.header { display: flex; justify-content: space-between; padding: 1rem; background: var(--accent-color); color: white; }
main { padding: 2rem; }
</style>