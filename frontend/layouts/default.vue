<template>
  <div class="app-shell">
    <AppHeader />
    <main class="page-shell">
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts">
import AppHeader from '~/components/AppHeader.vue';
</script>

<style scoped>
.app-shell {
  min-height: 100vh;
  background: radial-gradient(
      circle at top left,
      rgba(37, 99, 235, 0.24),
      transparent 28rem
    ),
    radial-gradient(
      circle at top right,
      rgba(16, 185, 129, 0.18),
      transparent 26rem
    ),
    #050b14;
}

.page-shell {
  padding: 3rem 2rem;
}
</style>
