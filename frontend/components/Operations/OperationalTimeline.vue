<script setup lang="ts">
const props = defineProps({
  events: {
    type: Array,
    default: () => [],
  },
});

function formatEvent(eventType: string) {
  return eventType.replace(/([a-z])([A-Z])/g, '$1 $2').replace(/_/g, ' ');
}

function eventGroup(eventType: string) {
  const normalized = eventType.toLowerCase();

  if (normalized.includes('contract')) return 'contract';
  if (normalized.includes('payment') || normalized.includes('paid'))
    return 'payment';
  if (normalized.includes('milestone')) return 'milestone';
  if (normalized.includes('activated') || normalized.includes('resumed'))
    return 'success';
  if (
    normalized.includes('overdue') ||
    normalized.includes('suspended') ||
    normalized.includes('disputed') ||
    normalized.includes('cancelled')
  )
    return 'risk';

  return 'system';
}
</script>

<template>
  <section class="portal-section timeline-section">
    <p class="eyebrow">Operational History</p>
    <h2>Engagement Timeline</h2>

    <div v-if="!events.length" class="empty-state">
      No operational events recorded yet.
    </div>

    <ol v-else class="timeline">
      <li
        v-for="event in events"
        :key="event.id"
        class="timeline-item"
        :class="`event-${eventGroup(event.event_type)}`"
      >
        <div class="timeline-dot" />

        <div class="timeline-card">
          <div class="timeline-card-header">
            <strong>{{ formatEvent(event.event_type) }}</strong>
            <span class="event-pill">{{ eventGroup(event.event_type) }}</span>
          </div>

          <p v-if="event.from_status || event.to_status">
            {{ event.from_status || '—' }} → {{ event.to_status || '—' }}
          </p>

          <small>{{ event.created_at }}</small>
        </div>
      </li>
    </ol>
  </section>
</template>

<style scoped>
.timeline-section {
  color: #e5eefc;
}

.timeline-section h2 {
  color: #f8fafc;
  margin: 0;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 800;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  margin-bottom: 10px;
}

.empty-state {
  color: #cbd5e1;
  margin-top: 16px;
}

.timeline {
  list-style: none;
  margin: 24px 0 0;
  padding: 0;
  position: relative;
}

.timeline::before {
  background: rgba(45, 212, 191, 0.24);
  content: '';
  height: 100%;
  left: 10px;
  position: absolute;
  top: 0;
  width: 2px;
}

.timeline-item {
  display: grid;
  gap: 14px;
  grid-template-columns: 24px 1fr;
  margin-bottom: 16px;
  position: relative;
}

.timeline-dot {
  border-radius: 999px;
  height: 20px;
  margin-top: 12px;
  width: 20px;
  z-index: 1;
}

.timeline-card {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 16px;
  background: rgba(8, 31, 42, 0.88);
  padding: 16px;
}

.timeline-card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.timeline-card strong {
  color: #f8fafc;
  display: block;
}

.timeline-card p {
  color: #cbd5e1;
  margin: 8px 0 6px;
}

.timeline-card small {
  color: #94a3b8;
}

.event-pill {
  border-radius: 999px;
  font-size: 0.68rem;
  font-weight: 800;
  letter-spacing: 0.08em;
  padding: 5px 9px;
  text-transform: uppercase;
}

.event-contract .timeline-dot,
.event-contract .event-pill {
  background: #38bdf8;
  color: #082f49;
}

.event-payment .timeline-dot,
.event-payment .event-pill {
  background: #34d399;
  color: #052e1b;
}

.event-milestone .timeline-dot,
.event-milestone .event-pill {
  background: #a78bfa;
  color: #2e1065;
}

.event-success .timeline-dot,
.event-success .event-pill {
  background: #22c55e;
  color: #052e16;
}

.event-risk .timeline-dot,
.event-risk .event-pill {
  background: #fb7185;
  color: #4c0519;
}

.event-system .timeline-dot,
.event-system .event-pill {
  background: #94a3b8;
  color: #0f172a;
}
</style>
