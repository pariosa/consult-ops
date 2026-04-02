<template>
  <div class="hero">
    <h1>ConsultantOps</h1>
    <p>Your full operations platform for consultants, clients, and projects</p>
    <button @click="$router.push('/demo-dashboard')">Try Demo</button>
  </div>
</template>

<style scoped>
.hero { text-align: center; padding: 4rem; }
button { padding: 1rem 2rem; background: #5c6bc0; color: white; border-radius: 8px; }
</style>