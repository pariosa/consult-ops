<template>
  <article class="kpi-tile ops-card">
    <p class="ops-eyebrow">{{ displayTitle }}</p>
    <p class="value">{{ value }}</p>
  </article>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(
  defineProps<{
    label?: string;
    title?: string;
    value?: string | number;
  }>(),
  {
    value: 0,
  },
);

const displayTitle = computed(() => props.title || props.label || 'Metric');
</script>

<style scoped>
.kpi-tile {
  min-height: 120px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding: 1.25rem;
  border: 1px solid rgba(85, 214, 190, 0.25);
  border-radius: 18px;
  background: rgba(8, 18, 31, 0.84);
  color: #e5f6ff;
}

.value {
  margin: 0.35rem 0 0;
  color: white;
  font-size: 2.25rem;
  font-weight: 800;
}
</style>
