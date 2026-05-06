<!-- components/Contracts/SoftwareContractPreview.vue -->
<script setup lang="ts">
defineProps<{
  body: string;
}>();
</script>

<template>
  <section class="portal-section">
    <div class="section-header">
      <h2>Contract Preview</h2>
      <p>Review generated software services language before sending.</p>
    </div>

    <pre class="contract-preview">{{ body }}</pre>
  </section>
</template>

<style scoped>
.contract-preview {
  white-space: pre-wrap;
  border: 1px solid rgba(85, 214, 190, 0.25);
  border-radius: 1rem;
  background: rgba(2, 6, 23, 0.85);
  color: #e5f6ff;
  padding: 1rem;
  line-height: 1.5;
  overflow-x: auto;
}
</style>
