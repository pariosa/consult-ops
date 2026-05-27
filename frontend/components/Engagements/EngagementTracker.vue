<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  status: string;
  platformFeeStatus?: string;
  milestones?: any[];
}>();

const normalizedStatus = computed(() => props.status || 'draft');

function milestoneComplete(milestone: any) {
  return ['approved', 'paid', 'completed'].includes(milestone?.status);
}
function milestoneTone(milestone: any) {
  const status = milestone?.status || '';

  if (['approved', 'paid', 'completed'].includes(status)) return 'complete';
  if (['rejected', 'disputed', 'failed'].includes(status)) return 'risk';
  if (['submitted', 'in_review', 'awaiting_review'].includes(status)) {
    return 'current';
  }

  return 'pending';
}
function stepTone(step: any, index: number) {
  if (step.complete) return 'complete';
  if (step.risk) return 'risk';
  if (step.current) return 'current';

  const firstIncompleteIndex = steps.value.findIndex((item) => !item.complete);

  if (index === firstIncompleteIndex) return 'current';

  return 'pending';
}
const baseSteps = computed(() => [
  {
    key: 'created',
    label: 'Created',
    detail: 'Engagement opened',
    complete: true,
  },
  {
    key: 'activated',
    label: 'Activated',
    detail: 'Billing cleared',
    complete:
      props.platformFeeStatus === 'paid' ||
      props.platformFeeStatus === 'waived' ||
      [
        'active',
        'work_in_progress',
        'awaiting_review',
        'paid',
        'completed',
      ].includes(normalizedStatus.value),
  },
  {
    key: 'contract_sent',
    label: 'Sent',
    detail: 'Agreement delivered',
    complete: [
      'contract_sent',
      'contract_signed',
      'active',
      'work_in_progress',
      'awaiting_review',
      'paid',
      'completed',
    ].includes(normalizedStatus.value),
  },
  {
    key: 'contract_signed',
    label: 'Signed',
    detail: 'Agreement accepted',
    complete: [
      'contract_signed',
      'active',
      'work_in_progress',
      'awaiting_review',
      'paid',
      'completed',
    ].includes(normalizedStatus.value),
  },
]);

const milestoneSteps = computed(() =>
  (props.milestones || []).map((milestone, index) => {
    const tone = milestoneTone(milestone);

    return {
      key: `milestone-${milestone.id || index}`,
      label: milestone.title || `Milestone ${index + 1}`,
      detail: milestone.status || 'not started',
      complete: tone === 'complete',
      risk: tone === 'risk',
      current: tone === 'current',
    };
  }),
);

const finalSteps = computed(() => [
  {
    key: 'paid',
    label: 'Paid',
    detail: 'Obligation settled',
    complete: ['paid', 'completed'].includes(normalizedStatus.value),
  },
  {
    key: 'completed',
    label: 'Complete',
    detail: 'Workflow closed',
    complete: normalizedStatus.value === 'completed',
  },
]);

const steps = computed(() => [
  ...baseSteps.value,
  ...milestoneSteps.value,
  ...finalSteps.value,
]);

const completedCount = computed(
  () => steps.value.filter((step) => step.complete).length,
);

const progressPercent = computed(() =>
  Math.round((completedCount.value / steps.value.length) * 100),
);
</script>

<template>
  <section class="battery-tracker">
    <div class="tracker-orbit" />

    <div class="tracker-header">
      <div>
        <p class="eyebrow">Workflow Charge</p>
        <h2>Engagement Power Track</h2>
        <p class="tracker-subtitle">
          Each charged cell represents an operational checkpoint completed.
        </p>
      </div>

      <div class="charge-readout">
        <strong>{{ progressPercent }}%</strong>
        <span>{{ completedCount }}/{{ steps.length }} charged</span>
      </div>
    </div>

    <div class="battery-shell">
      <div class="battery-body">
        <div
          v-for="(step, index) in steps"
          :key="step.key"
          class="battery-cell"
          :class="[
            {
              first: index === 0,
              last: index === steps.length - 1,
            },
            `tone-${stepTone(step, index)}`,
          ]"
        >
          <div class="cell-face">
            <div class="cell-shine" />
          </div>
        </div>
      </div>

      <div class="battery-cap">
        <div class="cap-light" />
      </div>
    </div>

    <ol class="battery-labels">
      <li
        v-for="(step, index) in steps"
        :key="`${step.key}-label`"
        class="battery-label"
        :class="[
          {
            complete: step.complete,
            first: index === 0,
            last: index === steps.length - 1,
          },
          `tone-${stepTone(step, index)}`,
        ]"
      >
        <strong>{{ step.label }}</strong>
        <span>{{ step.detail }}</span>
      </li>
    </ol>
  </section>
</template>

<style scoped>
.battery-tracker {
  position: relative;
  overflow: hidden;
  border: 1px solid rgba(45, 212, 191, 0.24);
  border-radius: 26px;
  background: radial-gradient(
      circle at 12% 0%,
      rgba(96, 165, 250, 0.18),
      transparent 34%
    ),
    radial-gradient(
      circle at 82% 10%,
      rgba(52, 211, 153, 0.14),
      transparent 32%
    ),
    linear-gradient(180deg, rgba(15, 23, 42, 0.98), rgba(2, 12, 23, 0.98));
  box-shadow:
    0 24px 80px rgba(0, 0, 0, 0.34),
    inset 0 1px 0 rgba(255, 255, 255, 0.04);
  color: #e5eefc;
  margin-bottom: 24px;
  padding: 26px;
}

.tracker-orbit {
  position: absolute;
  inset: auto -90px -140px auto;
  width: 280px;
  height: 280px;
  border-radius: 999px;
  background: rgba(45, 212, 191, 0.11);
  filter: blur(8px);
  pointer-events: none;
}

.tracker-header {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 18px;
  margin-bottom: 26px;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.14em;
  margin: 0 0 8px;
  text-transform: uppercase;
}

h2 {
  color: #f8fafc;
  font-size: clamp(1.35rem, 2vw, 1.8rem);
  letter-spacing: -0.03em;
  margin: 0;
}

.tracker-subtitle {
  color: #94a3b8;
  line-height: 1.5;
  margin: 8px 0 0;
}

.charge-readout {
  min-width: 132px;
  border: 1px solid rgba(45, 212, 191, 0.28);
  border-radius: 18px;
  background: rgba(2, 12, 23, 0.66);
  padding: 14px;
  text-align: right;
  box-shadow: inset 0 0 24px rgba(45, 212, 191, 0.06);
}

.charge-readout strong {
  display: block;
  background: linear-gradient(90deg, #60a5fa, #34d399);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  font-size: 1.55rem;
  font-weight: 950;
  line-height: 1;
}

.charge-readout span {
  color: #94a3b8;
  display: block;
  font-size: 0.74rem;
  font-weight: 800;
  margin-top: 5px;
  text-transform: uppercase;
}

.battery-shell {
  position: relative;
  z-index: 1;
  display: grid;
  grid-template-columns: 1fr 18px;
  gap: 7px;
  align-items: center;
}

.battery-body {
  position: relative;
  display: grid;
  grid-template-columns: repeat(v-bind('steps.length'), minmax(42px, 1fr));
  gap: 5px;
  border: 2px solid rgba(148, 163, 184, 0.22);
  border-radius: 18px;
  background: linear-gradient(
      180deg,
      rgba(2, 12, 23, 0.98),
      rgba(15, 23, 42, 0.84)
    ),
    repeating-linear-gradient(
      90deg,
      rgba(103, 232, 249, 0.05) 0,
      rgba(103, 232, 249, 0.05) 1px,
      transparent 1px,
      transparent 12px
    );
  padding: 7px;
  box-shadow:
    inset 0 0 28px rgba(0, 0, 0, 0.42),
    0 0 0 1px rgba(45, 212, 191, 0.08);
}

.battery-body::before {
  content: '';
  position: absolute;
  inset: -2px;
  border-radius: 18px;
  background: linear-gradient(
    90deg,
    rgba(96, 165, 250, 0.25),
    rgba(52, 211, 153, 0.18)
  );
  opacity: 0.24;
  pointer-events: none;
}

.battery-cell {
  position: relative;
  min-height: 72px;
  transform: skewX(-10deg);
  border: 1px solid rgba(148, 163, 184, 0.18);
  background: rgba(15, 23, 42, 0.88);
  overflow: hidden;
  transition:
    transform 0.18s ease,
    filter 0.18s ease,
    box-shadow 0.18s ease;
}

.battery-cell.first {
  border-radius: 12px 4px 4px 12px;
  clip-path: polygon(10% 0, 100% 0, 100% 100%, 0 100%, 0 18%);
}

.battery-cell.last {
  border-radius: 4px 12px 12px 4px;
  clip-path: polygon(0 0, 90% 0, 100% 18%, 100% 100%, 0 100%);
}

.battery-cell:not(.first):not(.last) {
  clip-path: polygon(8% 0, 100% 0, 92% 100%, 0 100%);
}

.battery-cell:hover {
  transform: skewX(-10deg) translateY(-3px);
  filter: brightness(1.12);
}

.cell-face {
  position: absolute;
  inset: 0;
  transform: skewX(10deg) scaleX(1.08);
  background: linear-gradient(
      180deg,
      rgba(255, 255, 255, 0.05),
      transparent 38%
    ),
    rgba(15, 23, 42, 0.8);
}

.battery-cell.tone-complete {
  border-color: rgba(167, 243, 208, 0.65);
  background: linear-gradient(90deg, #60a5fa, #34d399);
  box-shadow:
    0 0 22px rgba(52, 211, 153, 0.28),
    0 0 42px rgba(96, 165, 250, 0.14),
    inset 0 0 22px rgba(255, 255, 255, 0.18);
}

.battery-cell.tone-complete .cell-face {
  background: radial-gradient(
      circle at 24% 20%,
      rgba(255, 255, 255, 0.5),
      transparent 18%
    ),
    linear-gradient(135deg, rgba(96, 165, 250, 0.95), rgba(52, 211, 153, 0.92));
}

.cell-shine {
  position: absolute;
  inset: 0;
  background: linear-gradient(
    110deg,
    transparent 0%,
    transparent 28%,
    rgba(255, 255, 255, 0.2) 44%,
    transparent 60%,
    transparent 100%
  );
  opacity: 0;
  transform: translateX(-100%);
}

.battery-cell.tone-complete .cell-shine {
  animation: chargeSweep 2.8s ease-in-out infinite;
  opacity: 1;
}

.battery-cap {
  width: 18px;
  height: 46px;
  border: 2px solid rgba(148, 163, 184, 0.22);
  border-left: 0;
  border-radius: 0 10px 10px 0;
  background: rgba(15, 23, 42, 0.9);
  display: grid;
  place-items: center;
}

.cap-light {
  width: 6px;
  height: 24px;
  border-radius: 999px;
  background: linear-gradient(180deg, #60a5fa, #34d399);
  box-shadow: 0 0 18px rgba(52, 211, 153, 0.55);
  opacity: 0.9;
}

.battery-labels {
  position: relative;
  z-index: 1;
  display: grid;
  grid-template-columns: repeat(v-bind('steps.length'), minmax(42px, 1fr));
  gap: 5px;
  list-style: none;
  margin: 12px 25px 0 0;
  padding: 0;
}

.battery-label {
  color: #f8fafc;
  display: grid;
  gap: 4px;
  min-width: 0;
  text-align: center;
}

.battery-label strong {
  color: #f8fafc;
  font-size: clamp(0.62rem, 0.9vw, 0.78rem);
  font-weight: 950;
  line-height: 1.1;
  overflow-wrap: anywhere;
}

.battery-label.tone-complete strong {
  background: linear-gradient(90deg, #60a5fa, #34d399);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  filter: drop-shadow(0 0 8px rgba(52, 211, 153, 0.22));
}

.battery-label span {
  color: #94a3b8;
  font-size: 0.62rem;
  line-height: 1.2;
  overflow-wrap: anywhere;
}

.battery-label.tone-complete span {
  color: #a7f3d0;
}

@keyframes chargeSweep {
  0% {
    transform: translateX(-120%);
  }
  48% {
    transform: translateX(120%);
  }
  100% {
    transform: translateX(120%);
  }
}

@media (max-width: 820px) {
  .tracker-header {
    flex-direction: column;
  }

  .charge-readout {
    text-align: left;
  }

  .battery-shell,
  .battery-labels {
    min-width: 720px;
  }

  .battery-tracker {
    overflow-x: auto;
  }
}
.battery-cell.tone-current {
  border-color: rgba(250, 204, 21, 0.62);
  box-shadow:
    0 0 22px rgba(250, 204, 21, 0.22),
    inset 0 0 20px rgba(250, 204, 21, 0.08);
}

.battery-cell.tone-current .cell-face {
  background: linear-gradient(
    135deg,
    rgba(250, 204, 21, 0.28),
    rgba(15, 23, 42, 0.88)
  );
}

.battery-cell.tone-current {
  animation: currentPulse 2.2s ease-in-out infinite;
}

.battery-cell.tone-risk {
  border-color: rgba(251, 113, 133, 0.74);
  background: rgba(127, 29, 29, 0.42);
  box-shadow:
    0 0 24px rgba(251, 113, 133, 0.24),
    inset 0 0 18px rgba(251, 113, 133, 0.1);
}

.battery-label.tone-current strong {
  color: #fde68a;
}

.battery-label.tone-risk strong {
  color: #fecdd3;
}

@keyframes currentPulse {
  0%,
  100% {
    filter: brightness(1);
  }

  50% {
    filter: brightness(1.28);
  }
}
</style>
