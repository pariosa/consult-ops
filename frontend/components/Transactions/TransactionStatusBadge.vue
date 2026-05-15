<script setup lang="ts">
const props = defineProps<{
  status: string;
}>();

const statusClass = computed(() => {
  switch (props.status?.toLowerCase()) {
    case 'paid':
    case 'settled':
      return 'status-paid';
    case 'processing':
      return 'status-processing';
    case 'failed':
    case 'cancelled':
      return 'status-failed';
    default:
      return 'status-pending';
  }
});
</script>

<template>
  <span class="status-badge" :class="statusClass">
    {{ status }}
  </span>
</template>

<style scoped>
.status-badge {
  border-radius: 999px;
  display: inline-flex;
  font-size: 0.72rem;
  font-weight: 800;
  letter-spacing: 0.08em;
  padding: 6px 10px;
  text-transform: uppercase;
}

.status-pending {
  background: rgba(251, 191, 36, 0.16);
  color: #fbbf24;
}

.status-processing {
  background: rgba(96, 165, 250, 0.16);
  color: #93c5fd;
}

.status-paid {
  background: rgba(52, 211, 153, 0.16);
  color: #6ee7b7;
}

.status-failed {
  background: rgba(248, 113, 113, 0.16);
  color: #fca5a5;
}
</style>
