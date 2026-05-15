<script setup lang="ts">
const props = defineProps<{ status: string }>();

const badgeClass = computed(() => {
  switch (props.status) {
    case 'accepted':
      return 'accepted';
    case 'expired':
      return 'expired';
    default:
      return 'pending';
  }
});
</script>

<template>
  <span class="badge" :class="badgeClass">{{ status }}</span>
</template>

<style scoped>
.badge {
  border-radius: 999px;
  font-size: 0.7rem;
  font-weight: 900;
  letter-spacing: 0.08em;
  padding: 6px 10px;
  text-transform: uppercase;
}

.pending {
  background: rgba(251, 191, 36, 0.16);
  color: #fbbf24;
}

.accepted {
  background: rgba(52, 211, 153, 0.16);
  color: #6ee7b7;
}

.expired {
  background: rgba(248, 113, 113, 0.16);
  color: #fca5a5;
}
</style>
