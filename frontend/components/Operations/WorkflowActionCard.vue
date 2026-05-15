<script setup lang="ts">
const props = defineProps<{
  title: string;
  description?: string;
  status?: string;
  severity?: 'info' | 'success' | 'warning' | 'critical';
  primaryLabel?: string;
  secondaryLabel?: string;
  loading?: boolean;
}>();

const emit = defineEmits<{
  primary: [];
  secondary: [];
}>();

const severityClass = computed(() => {
  switch (props.severity) {
    case 'success':
      return 'severity-success';

    case 'warning':
      return 'severity-warning';

    case 'critical':
      return 'severity-critical';

    default:
      return 'severity-info';
  }
});
</script>

<template>
  <article class="workflow-card" :class="severityClass">
    <div class="workflow-card__header">
      <div>
        <p v-if="status" class="workflow-card__status">
          {{ status }}
        </p>

        <h3 class="workflow-card__title">
          {{ title }}
        </h3>
      </div>

      <slot name="badge" />
    </div>

    <p v-if="description" class="workflow-card__description">
      {{ description }}
    </p>

    <div class="workflow-card__actions">
      <button
        v-if="primaryLabel"
        class="workflow-card__button workflow-card__button--primary"
        :disabled="loading"
        @click="emit('primary')"
      >
        {{ primaryLabel }}
      </button>

      <button
        v-if="secondaryLabel"
        class="workflow-card__button workflow-card__button--secondary"
        :disabled="loading"
        @click="emit('secondary')"
      >
        {{ secondaryLabel }}
      </button>
    </div>
  </article>
</template>

<style scoped>
.workflow-card {
  border: 1px solid rgba(45, 212, 191, 0.2);
  border-radius: 18px;
  background: linear-gradient(
    180deg,
    rgba(15, 23, 42, 0.98),
    rgba(2, 12, 23, 0.98)
  );
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.22);
  overflow: hidden;
  padding: 20px;
  transition:
    transform 0.18s ease,
    border-color 0.18s ease,
    box-shadow 0.18s ease;
}

.workflow-card:hover {
  transform: translateY(-2px);
}

.workflow-card__header {
  align-items: flex-start;
  display: flex;
  gap: 12px;
  justify-content: space-between;
  margin-bottom: 12px;
}

.workflow-card__status {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 800;
  letter-spacing: 0.12em;
  margin: 0 0 6px;
  text-transform: uppercase;
}

.workflow-card__title {
  color: #f8fafc;
  font-size: 1.05rem;
  font-weight: 800;
  line-height: 1.3;
  margin: 0;
}

.workflow-card__description {
  color: #cbd5e1;
  line-height: 1.55;
  margin: 0 0 18px;
}

.workflow-card__actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.workflow-card__button {
  border: 0;
  border-radius: 12px;
  cursor: pointer;
  font-weight: 800;
  padding: 11px 14px;
  transition:
    opacity 0.15s ease,
    transform 0.15s ease;
}

.workflow-card__button:hover {
  transform: translateY(-1px);
}

.workflow-card__button:disabled {
  cursor: not-allowed;
  opacity: 0.65;
}

.workflow-card__button--primary {
  background: linear-gradient(90deg, #60a5fa, #34d399);
  color: #04111f;
}

.workflow-card__button--secondary {
  background: rgba(15, 23, 42, 0.96);
  border: 1px solid rgba(45, 212, 191, 0.28);
  color: #e5eefc;
}

.severity-info {
  border-color: rgba(96, 165, 250, 0.32);
}

.severity-success {
  border-color: rgba(52, 211, 153, 0.4);
  box-shadow: 0 12px 40px rgba(16, 185, 129, 0.08);
}

.severity-warning {
  border-color: rgba(251, 191, 36, 0.45);
  box-shadow: 0 12px 40px rgba(245, 158, 11, 0.08);
}

.severity-critical {
  border-color: rgba(248, 113, 113, 0.55);
  box-shadow: 0 12px 40px rgba(239, 68, 68, 0.12);
}
</style>
