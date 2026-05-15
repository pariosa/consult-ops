<script setup lang="ts">
import { computed, onMounted, ref, onUnmounted } from 'vue';
import { useApi } from '~/composables/useApi';
import { useOperationalEvents } from '~/composables/useOperationalEvents';
import OperationalTimeline from '~/components/Operations/OperationalTimeline.vue';
import WorkflowActionCard from '~/components/Operations/WorkflowActionCard.vue';
import { useEngagementMilestones } from '~/composables/useEngagementMilestones';
const { get } = useApi();
const { getOrganizationEvents } = useOperationalEvents();

const loading = ref(true);
const error = ref('');
const organization = ref<any>(null);
const events = ref<any[]>([]);
const activeFilter = ref('all');
const { approveMilestone } = useEngagementMilestones();
const actionLoadingId = ref<number | null>(null);
async function approveFromDashboard(event: any) {
  const milestoneId = milestoneIdFromEvent(event);

  console.log('approve milestone id', milestoneId);

  if (!milestoneId) {
    error.value = 'Unable to find milestone id.';
    return;
  }

  await approveMilestone(milestoneId);
  await refresh();
}
function eventMetadata(event: any) {
  try {
    return typeof event.metadata === 'string'
      ? JSON.parse(event.metadata || '{}')
      : event.metadata || {};
  } catch {
    return {};
  }
}

function milestoneIdFromEvent(event: any): number | null {
  const metadata = eventMetadata(event);

  if (metadata.milestone_id !== undefined && metadata.milestone_id !== null) {
    return Number(metadata.milestone_id);
  }

  if (event.entity_type === 'milestone') {
    return Number(event.entity_id);
  }

  return null;
}

function milestoneTitleFromEvent(event: any) {
  const metadata = eventMetadata(event);

  return metadata.milestone_title || metadata.title || 'Submitted milestone';
}

function amountFromEvent(event: any) {
  const metadata = eventMetadata(event);

  return metadata.amount_cents
    ? formatMoney(Number(metadata.amount_cents))
    : null;
}

const filters = [
  { key: 'all', label: 'All' },
  { key: 'payment', label: 'Payments' },
  { key: 'milestone', label: 'Milestones' },
  { key: 'contract', label: 'Contracts' },
  { key: 'risk', label: 'Risk' },
  { key: 'system', label: 'System' },
];

let refreshTimer: ReturnType<typeof setInterval> | null = null;

function eventGroup(eventType: string) {
  const value = eventType?.toLowerCase?.() || '';

  if (value.includes('contract')) return 'contract';
  if (
    value.includes('payment') ||
    value.includes('paid') ||
    value.includes('billing') ||
    value.includes('fee')
  )
    return 'payment';
  if (value.includes('milestone')) return 'milestone';
  if (
    value.includes('disputed') ||
    value.includes('cancelled') ||
    value.includes('overdue') ||
    value.includes('suspended')
  )
    return 'risk';
  return 'system';
}

function eventSeverity(eventType: string) {
  const value = eventType?.toLowerCase?.() || '';

  if (value.includes('disputed') || value.includes('cancelled'))
    return 'critical';
  if (value.includes('overdue') || value.includes('suspended'))
    return 'warning';
  if (
    value.includes('paid') ||
    value.includes('approved') ||
    value.includes('activated')
  )
    return 'success';
  return 'info';
}
const awaitingPayment = computed(() =>
  events.value.filter((event) => {
    const type = event.event_type || '';
    const status = event.to_status || '';
    const metadata = eventMetadata(event);
    const billingId = metadata.billing_id ? Number(metadata.billing_id) : null;

    return (
      [
        'ActivationFeeCreated',
        'EngagementBillingCreated',
        'ActivationCheckoutStarted',
      ].includes(type) &&
      status !== 'paid' &&
      billingId &&
      !paidBillingIds.value.has(billingId)
    );
  }),
);
const filteredEvents = computed(() => {
  if (activeFilter.value === 'all') return events.value;

  return events.value.filter(
    (event) => eventGroup(event.event_type) === activeFilter.value,
  );
});

const approvedMilestoneIds = computed(() => {
  return new Set(
    events.value
      .filter((event) => event.event_type === 'MilestoneApproved')
      .map((event) => milestoneIdFromEvent(event))
      .filter(Boolean),
  );
});
const paidBillingIds = computed(() => {
  return new Set(
    events.value
      .filter((event) => event.event_type === 'ActivationFeePaid')
      .map((event) => {
        const metadata = eventMetadata(event);
        return metadata.billing_id ? Number(metadata.billing_id) : null;
      })
      .filter(Boolean),
  );
});
const needsApproval = computed(() =>
  events.value.filter((event) => {
    if (event.event_type !== 'MilestoneSubmitted') return false;

    const milestoneId = milestoneIdFromEvent(event);

    return milestoneId && !approvedMilestoneIds.value.has(milestoneId);
  }),
);
const pendingSignature = computed(() =>
  events.value.filter((event) =>
    ['EngagementContractSent'].includes(event.event_type),
  ),
);

const riskQueue = computed(() =>
  events.value.filter(
    (event) =>
      eventSeverity(event.event_type) === 'critical' ||
      eventSeverity(event.event_type) === 'warning',
  ),
);

const revenueCents = computed(() => {
  return paidEvents.value.reduce((total, event) => {
    try {
      const metadata =
        typeof event.metadata === 'string'
          ? JSON.parse(event.metadata || '{}')
          : event.metadata || {};

      return total + Number(metadata.amount_cents || 0);
    } catch {
      return total;
    }
  }, 0);
});

function engagementIdFromEvent(event: any) {
  if (event.entity_type === 'engagement') return event.entity_id;

  try {
    const metadata =
      typeof event.metadata === 'string'
        ? JSON.parse(event.metadata || '{}')
        : event.metadata || {};

    return metadata.engagement_id || null;
  } catch {
    return null;
  }
}
function formatMoney(cents: number) {
  return `$${(cents / 100).toFixed(2)}`;
}

function eventLink(event: any) {
  if (event.entity_type === 'engagement') {
    return `/engagements/${event.entity_id}`;
  }

  try {
    const metadata =
      typeof event.metadata === 'string'
        ? JSON.parse(event.metadata || '{}')
        : event.metadata || {};

    if (metadata.engagement_id) {
      return `/engagements/${metadata.engagement_id}`;
    }
  } catch {}

  return null;
}

const pendingPayments = computed(() =>
  events.value.filter(
    (event) =>
      [
        'ActivationFeeCreated',
        'EngagementBillingCreated',
        'ActivationCheckoutStarted',
      ].includes(event.event_type) && event.to_status !== 'paid',
  ),
);

const paidEvents = computed(() =>
  events.value.filter((event) =>
    ['ActivationFeePaid', 'PaymentCreated', 'MilestonePaid'].includes(
      event.event_type,
    ),
  ),
);

const milestoneEvents = computed(() =>
  events.value.filter((event) => event.event_type?.includes('Milestone')),
);

const activationEvents = computed(() =>
  events.value.filter((event) =>
    ['EngagementActivated', 'PaymentReceived', 'ActivationFeePaid'].includes(
      event.event_type,
    ),
  ),
);

const riskEvents = computed(() =>
  events.value.filter((event) =>
    [
      'EngagementDisputed',
      'EngagementCancelled',
      'EngagementOverdue',
      'EngagementSuspended',
    ].includes(event.event_type),
  ),
);

async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    organization.value = await get('/api/me/organization');

    if (!organization.value?.id) {
      error.value = 'No organization found.';
      events.value = [];
      return;
    }

    events.value = await getOrganizationEvents(organization.value.id);
  } catch (err: any) {
    error.value = err?.message || 'Failed to load operations dashboard.';
  } finally {
    loading.value = false;
  }
}

onMounted(() => {
  refresh();

  refreshTimer = setInterval(refresh, 15000);
});

onUnmounted(() => {
  if (refreshTimer) clearInterval(refreshTimer);
});
</script>

<template>
  <DashboardShell
    title="Operations Dashboard"
    subtitle="Monitor payments, milestones, engagement activity, and operational history."
  >
    <section v-if="loading" class="portal-section">
      Loading operations dashboard...
    </section>

    <section v-else-if="error" class="form-error">
      {{ error }}
    </section>

    <template v-else>
      <section class="portal-section hero-section">
        <p class="eyebrow">Organization Command Center</p>
        <h2>{{ organization?.name }}</h2>
        <p>
          A live operational view of engagement activity, billing, milestone
          progress, and workflow validation.
        </p>
      </section>

      <section class="stats-grid">
        <div class="ops-stat">
          <p>Pending Payments</p>
          <strong>{{ pendingPayments.length }}</strong>
        </div>

        <div class="ops-stat">
          <p>Paid Events</p>
          <strong>{{ paidEvents.length }}</strong>
        </div>

        <div class="ops-stat">
          <p>Milestone Activity</p>
          <strong>{{ milestoneEvents.length }}</strong>
        </div>

        <div class="ops-stat">
          <p>Activations</p>
          <strong>{{ activationEvents.length }}</strong>
        </div>

        <div class="ops-stat risk">
          <p>Risk Events</p>
          <strong>{{ riskEvents.length }}</strong>
        </div>
        <div class="ops-stat">
          <p>Recorded Revenue</p>
          <strong>{{ formatMoney(revenueCents) }}</strong>
        </div>
      </section>
      <section class="queue-grid">
        <div class="queue-card">
          <p class="eyebrow">Needs Approval</p>
          <h3>{{ needsApproval.length }}</h3>
          <p>Milestones waiting for review.</p>
        </div>

        <div class="queue-card">
          <p class="eyebrow">Awaiting Payment</p>
          <h3>{{ awaitingPayment.length }}</h3>
          <p>Billing items not yet paid.</p>
        </div>

        <div class="queue-card">
          <p class="eyebrow">Pending Signature</p>
          <h3>{{ pendingSignature.length }}</h3>
          <p>Contracts sent but not completed.</p>
        </div>

        <div class="queue-card risk">
          <p class="eyebrow">Operational Risk</p>
          <h3>{{ riskQueue.length }}</h3>
          <p>Disputes, overdue items, or suspended workflows.</p>
        </div>
      </section>
      <section class="portal-section">
        <div class="section-header">
          <div>
            <p class="eyebrow">Operational Actions</p>
            <h2>Workflow Action Center</h2>
          </div>
        </div>
        <div
          v-if="!needsApproval.length && !awaitingPayment.length"
          class="empty-state"
        >
          No workflow actions currently need attention.
        </div>
        <div v-else class="workflow-grid">
          <WorkflowActionCard
            v-for="event in needsApproval.slice(0, 4)"
            :key="event.id"
            title="Milestone Awaiting Approval"
            :description="`${milestoneTitleFromEvent(event)}${amountFromEvent(event) ? ` · ${amountFromEvent(event)}` : ''}`"
            status="Needs Approval"
            severity="warning"
            primary-label="Approve"
            secondary-label="Open"
            :loading="actionLoadingId === event.id"
            @primary="approveFromDashboard(event)"
            @secondary="
              engagementIdFromEvent(event) &&
                navigateTo(`/engagements/${engagementIdFromEvent(event)}`)
            "
          />
          <WorkflowActionCard
            v-for="event in awaitingPayment.slice(0, 4)"
            :key="`payment-${event.id}`"
            title="Awaiting Payment"
            :description="`Billing event: ${event.event_type}`"
            status="Payment Required"
            severity="critical"
            primary-label="Open Billing"
            secondary-label="View Engagement"
            @primary="
              engagementIdFromEvent(event) &&
                navigateTo(`/engagements/${engagementIdFromEvent(event)}`)
            "
            @secondary="
              engagementIdFromEvent(event) &&
                navigateTo(`/engagements/${engagementIdFromEvent(event)}`)
            "
          />
        </div>
      </section>
      <section class="portal-section">
        <div class="section-header">
          <div>
            <p class="eyebrow">Recent Activity</p>
            <h2>Operational Event Feed</h2>
            <p>
              Every major system action is recorded here for accountability and
              traceability.
            </p>
          </div>

          <button class="form-button secondary" @click="refresh">
            Refresh
          </button>
        </div>
        <div class="filter-row">
          <button
            v-for="filter in filters"
            :key="filter.key"
            class="filter-pill"
            :class="{ active: activeFilter === filter.key }"
            @click="activeFilter = filter.key"
          >
            {{ filter.label }}
          </button>
        </div>
        <OperationalTimeline :events="filteredEvents" />
      </section>
    </template>
  </DashboardShell>
</template>

<style scoped>
.queue-grid {
  display: grid;
  gap: 14px;
  margin-bottom: 24px;
}

.queue-card {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 16px;
  background: rgba(8, 31, 42, 0.9);
  padding: 18px;
}

.queue-card h3 {
  color: #f8fafc;
  font-size: 2rem;
  margin: 4px 0;
}

.queue-card p {
  color: #cbd5e1;
  margin: 0;
}

.queue-card.risk {
  border-color: rgba(251, 113, 133, 0.38);
}

.filter-row {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin: 18px 0 22px;
}

.filter-pill {
  border: 1px solid rgba(45, 212, 191, 0.28);
  border-radius: 999px;
  background: rgba(8, 31, 42, 0.9);
  color: #cbd5e1;
  cursor: pointer;
  font-weight: 800;
  padding: 8px 12px;
}

.filter-pill.active {
  background: linear-gradient(90deg, #60a5fa, #34d399);
  color: #04111f;
}

.event-link-list {
  display: grid;
  gap: 10px;
  margin-top: 20px;
}

.event-link-card {
  border: 1px solid rgba(45, 212, 191, 0.18);
  border-radius: 12px;
  background: rgba(2, 12, 23, 0.75);
  color: #e5eefc;
  display: flex;
  justify-content: space-between;
  gap: 12px;
  padding: 12px;
  text-decoration: none;
}

.event-link-card:hover {
  border-color: rgba(52, 211, 153, 0.55);
}

.event-link-card span {
  font-weight: 800;
}

.event-link-card small {
  color: #94a3b8;
}

@media (min-width: 760px) {
  .queue-grid {
    grid-template-columns: repeat(4, minmax(0, 1fr));
  }
}
.portal-section {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 18px;
  background: linear-gradient(
    180deg,
    rgba(15, 23, 42, 0.96),
    rgba(2, 12, 23, 0.96)
  );
  box-shadow: 0 18px 50px rgba(0, 0, 0, 0.28);
  color: #e5eefc;
  padding: 24px;
  margin-bottom: 24px;
}

.hero-section h2,
.portal-section h2 {
  color: #f8fafc;
  margin: 0 0 10px;
}

.hero-section p,
.portal-section p {
  color: #cbd5e1;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 800;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  margin-bottom: 12px;
}

.stats-grid {
  display: grid;
  gap: 14px;
  margin-bottom: 24px;
}

.ops-stat {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 16px;
  background: rgba(8, 31, 42, 0.9);
  padding: 18px;
}

.ops-stat p {
  color: #94a3b8;
  font-size: 0.82rem;
  font-weight: 800;
  letter-spacing: 0.08em;
  margin: 0 0 10px;
  text-transform: uppercase;
}

.ops-stat strong {
  color: #f8fafc;
  font-size: 2rem;
}

.ops-stat.risk {
  border-color: rgba(251, 113, 133, 0.35);
}

.section-header {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  align-items: flex-start;
  margin-bottom: 18px;
}

.form-button {
  border: 0;
  border-radius: 12px;
  background: linear-gradient(90deg, #60a5fa, #34d399);
  color: #04111f;
  cursor: pointer;
  font-weight: 800;
  padding: 12px 16px;
}

.form-button.secondary {
  background: rgba(15, 23, 42, 0.95);
  border: 1px solid rgba(45, 212, 191, 0.32);
  color: #e5eefc;
}

.form-error {
  border: 1px solid rgba(248, 113, 113, 0.38);
  border-radius: 14px;
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
  padding: 16px;
}

@media (min-width: 760px) {
  .stats-grid {
    grid-template-columns: repeat(5, minmax(0, 1fr));
  }
}
.workflow-grid {
  display: grid;
  gap: 16px;
}

@media (min-width: 960px) {
  .workflow-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
</style>
