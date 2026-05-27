<script setup lang="ts">
import { computed } from 'vue';
import { getEngagementNextAction } from '~/composables/useEngagementNextAction';

const props = defineProps<{
  engagement: any;
}>();

const action = computed(() => getEngagementNextAction(props.engagement));

const toneClass = computed(() => `tone-${action.value.tone}`);
</script>

<template>
  <section class="next-action-card" :class="toneClass">
    <div class="next-action-glow" />

    <div class="next-action-content">
      <div class="next-action-header">
        <p class="eyebrow">Recommended Next Action</p>
        <span class="status-pill">{{ action.label }}</span>
      </div>

      <h2>{{ action.title }}</h2>

      <p class="description">
        {{ action.description }}
      </p>

      <div class="action-row">
        <NuxtLink
          v-if="action.primaryLabel && action.primaryTo"
          :to="action.primaryTo"
          class="primary-action"
        >
          {{ action.primaryLabel }}
        </NuxtLink>

        <NuxtLink
          v-if="action.secondaryLabel && action.secondaryTo"
          :to="action.secondaryTo"
          class="secondary-action"
        >
          {{ action.secondaryLabel }}
        </NuxtLink>
      </div>
    </div>
  </section>
</template>

<style scoped>
.next-action-card {
  position: relative;
  overflow: hidden;
  border: 1px solid rgba(45, 212, 191, 0.24);
  border-radius: 22px;
  background:
    linear-gradient(135deg, rgba(15, 23, 42, 0.98), rgba(2, 12, 23, 0.98))
      padding-box,
    linear-gradient(135deg, rgba(96, 165, 250, 0.8), rgba(52, 211, 153, 0.75))
      border-box;
  color: #e5eefc;
  padding: 24px;
  box-shadow: 0 24px 70px rgba(0, 0, 0, 0.26);
}

.next-action-glow {
  position: absolute;
  inset: -40% auto auto -20%;
  width: 260px;
  height: 260px;
  border-radius: 999px;
  background: rgba(45, 212, 191, 0.15);
  filter: blur(8px);
  pointer-events: none;
}

.next-action-content {
  position: relative;
  z-index: 1;
}

.next-action-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 12px;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.12em;
  margin: 0;
  text-transform: uppercase;
}

.status-pill {
  border-radius: 999px;
  font-size: 0.7rem;
  font-weight: 900;
  letter-spacing: 0.08em;
  padding: 7px 10px;
  text-transform: uppercase;
}

h2 {
  color: #f8fafc;
  font-size: clamp(1.35rem, 2vw, 1.8rem);
  line-height: 1.1;
  margin: 0;
}

.description {
  color: #cbd5e1;
  line-height: 1.65;
  margin: 14px 0 0;
  max-width: 780px;
}

.action-row {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-top: 20px;
}

.primary-action,
.secondary-action {
  border-radius: 999px;
  font-weight: 900;
  padding: 11px 15px;
  text-decoration: none;
}

.primary-action {
  background: linear-gradient(90deg, #60a5fa, #34d399);
  color: #04111f;
}

.secondary-action {
  border: 1px solid rgba(103, 232, 249, 0.28);
  color: #cde7ff;
}

.tone-setup .status-pill {
  background: #38bdf8;
  color: #082f49;
}

.tone-waiting .status-pill {
  background: #facc15;
  color: #422006;
}

.tone-success .status-pill {
  background: #34d399;
  color: #052e1b;
}

.tone-risk .status-pill {
  background: #fb7185;
  color: #4c0519;
}

.tone-finance .status-pill {
  background: #a78bfa;
  color: #2e1065;
}
</style>
