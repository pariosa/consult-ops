<!-- pages/admin/index.vue -->
<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import DashboardShell from '~/components/DashboardShell.vue';
import { useApi } from '~/composables/useApi';

definePageMeta({
  middleware: ['role'],
  allowedUserTypes: ['admin', 'super_admin'],
});

const api = useApi();

const loading = ref(true);
const searching = ref(false);
const error = ref('');
const search = ref('');
const searchResults = ref<any>({
  organizations: [],
  users: [],
  projects: [],
  clients: [],
  engagements: [],
});

const dashboard = ref<any>({
  system_totals: {
    organizations: 0,
    users: 0,
    projects: 0,
    clients: 0,
    engagements: 0,
    contracts: 0,
    transactions: 0,
  },
  overview: {
    organizations: 0,
    users: 0,
    active_engagements: 0,
    awaiting_payment: 0,
    signed_contracts: 0,
    transactions_cents_30d: 0,
    activation_fees_cents_30d: 0,
    completed_engagements: 0,
  },
  revenue: {
    activation_fees_30d: 0,
    activation_fees_prev_30d: 0,
    transactions_30d: 0,
    transactions_prev_30d: 0,
  },
  health: {
    database: 'unknown',
    stripe: 'unknown',
    email: 'unknown',
    webhooks: 'unknown',
  },
  engagement_statuses: {
    draft: 0,
    pending_signature: 0,
    awaiting_payment: 0,
    active: 0,
    completed: 0,
    disputed: 0,
  },
  adoption: {
    organizations: 0,
    with_projects: 0,
    with_engagements: 0,
    with_signed_contracts: 0,
    with_paid_transactions: 0,
  },
  action_queue: [],
  top_organizations: [],
  recent_activity: [],
});

const quickActions = [
  {
    label: 'User Management',
    description: 'Create users, inspect accounts, and manage platform roles.',
    to: '/admin/users',
  },
  {
    label: 'Organizations',
    description:
      'Review workspace ownership, membership, and organization health.',
    to: '/organization',
  },
  {
    label: 'Projects',
    description: 'Inspect project pipelines and engagement readiness.',
    to: '/organization/projects',
  },
  {
    label: 'Clients',
    description: 'Review client records and verified payment parties.',
    to: '/organization/clients',
  },
  {
    label: 'Notifications',
    description: 'Review system notices and operational messages.',
    to: '/notifications',
  },
  {
    label: 'Operational Finance',
    description:
      'Review party balances, transactions, and settlement movement.',
    to: '/organization/finance',
  },
];

const groupedSearchResults = computed(() => {
  return [
    { label: 'Organizations', items: searchResults.value.organizations || [] },
    { label: 'Users', items: searchResults.value.users || [] },
    { label: 'Projects', items: searchResults.value.projects || [] },
    { label: 'Clients', items: searchResults.value.clients || [] },
    { label: 'Engagements', items: searchResults.value.engagements || [] },
  ].filter((group) => group.items.length);
});

const hasSearchResults = computed(() => groupedSearchResults.value.length > 0);

const healthScore = computed(() => {
  const statuses = dashboard.value.health || {};
  const values = Object.values(statuses);
  if (!values.length) return 0;

  const healthy = values.filter((value) =>
    ['healthy', 'configured'].includes(String(value)),
  ).length;

  return Math.round((healthy / values.length) * 100);
});

const healthLabel = computed(() => {
  if (healthScore.value >= 90) return 'Healthy';
  if (healthScore.value >= 70) return 'Watch';
  return 'Needs attention';
});

const activationFeeChange = computed(() =>
  percentChange(
    dashboard.value.revenue.activation_fees_30d,
    dashboard.value.revenue.activation_fees_prev_30d,
  ),
);

const transactionChange = computed(() =>
  percentChange(
    dashboard.value.revenue.transactions_30d,
    dashboard.value.revenue.transactions_prev_30d,
  ),
);

function formatMoney(cents: number) {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
  }).format((Number(cents) || 0) / 100);
}

function percentChange(current: number, previous: number) {
  const now = Number(current) || 0;
  const before = Number(previous) || 0;

  if (!before && !now) return 0;
  if (!before) return 100;

  return Math.round(((now - before) / before) * 100);
}

function formatEventName(value: string) {
  return String(value || '')
    .replace(/([a-z])([A-Z])/g, '$1 $2')
    .replace(/_/g, ' ');
}

function statusClass(value: string) {
  const normalized = String(value || '').toLowerCase();

  if (['healthy', 'configured'].includes(normalized)) return 'good';
  if (['unknown', 'warning', 'watch'].includes(normalized)) return 'watch';

  return 'bad';
}

function adoptionPercent(value: number) {
  const total = Number(dashboard.value.adoption.organizations) || 0;
  if (!total) return 0;

  return Math.round(((Number(value) || 0) / total) * 100);
}

async function loadDashboard() {
  loading.value = true;
  error.value = '';

  try {
    const data = await api.get('/api/platform/dashboard');
    dashboard.value = {
      ...dashboard.value,
      ...data,
    };
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to load platform dashboard.';
  } finally {
    loading.value = false;
  }
}

let searchTimer: ReturnType<typeof setTimeout> | null = null;

watch(search, (value) => {
  if (searchTimer) clearTimeout(searchTimer);

  const query = value.trim();

  if (query.length < 2) {
    searchResults.value = {
      organizations: [],
      users: [],
      projects: [],
      clients: [],
      engagements: [],
    };
    return;
  }

  searchTimer = setTimeout(async () => {
    searching.value = true;

    try {
      searchResults.value = await api.get(
        `/api/platform/search?q=${encodeURIComponent(query)}`,
      );
    } catch {
      searchResults.value = {
        organizations: [],
        users: [],
        projects: [],
        clients: [],
        engagements: [],
      };
    } finally {
      searching.value = false;
    }
  }, 250);
});

onMounted(loadDashboard);
</script>

<template>
  <DashboardShell
    title="Platform Admin"
    subtitle="Monitor platform health, revenue movement, user adoption, and operational risk."
  >
    <section class="search-panel">
      <div>
        <p class="eyebrow">Admin Search</p>
        <h2>Find anything on the platform</h2>
      </div>

      <input
        v-model="search"
        class="search-input"
        placeholder="Search organizations, users, projects, clients, engagements..."
      />

      <div v-if="searching" class="search-empty">Searching...</div>

      <div
        v-else-if="search.trim().length >= 2 && !hasSearchResults"
        class="search-empty"
      >
        No matching platform records found.
      </div>

      <div v-else-if="hasSearchResults" class="search-results">
        <div
          v-for="group in groupedSearchResults"
          :key="group.label"
          class="search-group"
        >
          <p>{{ group.label }}</p>

          <NuxtLink
            v-for="item in group.items"
            :key="`${item.type}-${item.id}`"
            :to="item.route"
            class="search-result"
          >
            <strong>{{ item.label }}</strong>
            <span>{{ item.description || item.type }}</span>
          </NuxtLink>
        </div>
      </div>
    </section>

    <section v-if="loading" class="panel">Loading platform overview...</section>

    <section v-else-if="error" class="error-panel">
      {{ error }}
    </section>

    <template v-else>
      <section class="hero-grid">
        <div class="health-card">
          <p class="eyebrow">Platform Health</p>
          <div class="health-score">{{ healthScore }}%</div>
          <p class="health-label">{{ healthLabel }}</p>
          <p class="muted">Database, Stripe, email, and webhook readiness.</p>
        </div>

        <div class="stat-card">
          <p class="eyebrow">Organizations</p>
          <h2>{{ dashboard.overview.organizations }}</h2>
          <p class="muted">Total workspaces on the platform.</p>
        </div>

        <div class="stat-card">
          <p class="eyebrow">Users</p>
          <h2>{{ dashboard.overview.users }}</h2>
          <p class="muted">Registered platform accounts.</p>
        </div>

        <div class="stat-card">
          <p class="eyebrow">Active Engagements</p>
          <h2>{{ dashboard.overview.active_engagements }}</h2>
          <p class="muted">
            {{ dashboard.overview.awaiting_payment }} awaiting payment.
          </p>
        </div>
      </section>

      <section class="metric-grid">
        <div class="stat-card">
          <p class="eyebrow">Signed Contracts</p>
          <h2>{{ dashboard.overview.signed_contracts }}</h2>
          <p class="muted">Commitments accepted by parties.</p>
        </div>

        <div class="stat-card">
          <p class="eyebrow">Transactions 30D</p>
          <h2>{{ formatMoney(dashboard.overview.transactions_cents_30d) }}</h2>
          <p class="muted">
            {{ transactionChange >= 0 ? '+' : '' }}{{ transactionChange }}% vs
            previous 30D.
          </p>
        </div>

        <div class="stat-card">
          <p class="eyebrow">Activation Fees 30D</p>
          <h2>
            {{ formatMoney(dashboard.overview.activation_fees_cents_30d) }}
          </h2>
          <p class="muted">
            {{ activationFeeChange >= 0 ? '+' : '' }}{{ activationFeeChange }}%
            vs previous 30D.
          </p>
        </div>

        <div class="stat-card">
          <p class="eyebrow">Completed Engagements</p>
          <h2>{{ dashboard.overview.completed_engagements }}</h2>
          <p class="muted">Closed-out operational workflows.</p>
        </div>
      </section>

      <section class="panel">
        <p class="eyebrow">System Totals</p>
        <h2>Platform snapshot</h2>

        <div class="totals-grid">
          <div
            v-for="(value, key) in dashboard.system_totals"
            :key="key"
            class="mini-stat"
          >
            <span>{{ String(key).replace(/_/g, ' ') }}</span>
            <strong>{{ value }}</strong>
          </div>
        </div>
      </section>

      <section class="two-column">
        <div class="panel">
          <div class="section-heading">
            <div>
              <p class="eyebrow">Platform Action Queue</p>
              <h2>What needs attention</h2>
            </div>

            <button class="ghost-button" @click="loadDashboard">Refresh</button>
          </div>

          <div v-if="!dashboard.action_queue.length" class="empty-state">
            No current action items. Platform operations look clear.
          </div>

          <NuxtLink
            v-for="item in dashboard.action_queue"
            :key="item.type"
            :to="item.route || '/admin'"
            class="alert-row"
          >
            <div>
              <strong>{{ item.label }}</strong>
              <p>{{ item.description }}</p>
            </div>
            <span>{{ item.count }}</span>
          </NuxtLink>
        </div>

        <div class="panel">
          <p class="eyebrow">User Adoption</p>
          <h2>Workspace activation funnel</h2>

          <div class="funnel-row">
            <span>Organizations</span>
            <strong>{{ dashboard.adoption.organizations }}</strong>
          </div>

          <div class="funnel-row">
            <span>With projects</span>
            <strong>
              {{ dashboard.adoption.with_projects }}
              <small
                >{{ adoptionPercent(dashboard.adoption.with_projects) }}%</small
              >
            </strong>
          </div>

          <div class="funnel-row">
            <span>With engagements</span>
            <strong>
              {{ dashboard.adoption.with_engagements }}
              <small
                >{{
                  adoptionPercent(dashboard.adoption.with_engagements)
                }}%</small
              >
            </strong>
          </div>

          <div class="funnel-row">
            <span>With signed contracts</span>
            <strong>
              {{ dashboard.adoption.with_signed_contracts }}
              <small
                >{{
                  adoptionPercent(dashboard.adoption.with_signed_contracts)
                }}%</small
              >
            </strong>
          </div>

          <div class="funnel-row">
            <span>With paid transactions</span>
            <strong>
              {{ dashboard.adoption.with_paid_transactions }}
              <small
                >{{
                  adoptionPercent(dashboard.adoption.with_paid_transactions)
                }}%</small
              >
            </strong>
          </div>
        </div>
      </section>

      <section class="two-column">
        <div class="panel">
          <p class="eyebrow">Engagement Status Breakdown</p>
          <h2>Workflow distribution</h2>

          <div
            v-for="(count, status) in dashboard.engagement_statuses"
            :key="status"
            class="status-row"
          >
            <span>{{ String(status).replace(/_/g, ' ') }}</span>
            <strong>{{ count }}</strong>
          </div>
        </div>

        <div class="panel">
          <p class="eyebrow">Top Organizations</p>
          <h2>Power users</h2>

          <div v-if="!dashboard.top_organizations.length" class="empty-state">
            No organization activity yet.
          </div>

          <NuxtLink
            v-for="org in dashboard.top_organizations"
            :key="org.id"
            :to="`/platform/organizations/${org.id}`"
            class="org-row"
          >
            <div>
              <strong>{{ org.name }}</strong>
              <p>{{ org.engagement_count }} engagements</p>
            </div>
            <span>{{ formatMoney(org.transaction_volume_cents) }}</span>
          </NuxtLink>
        </div>
      </section>

      <section class="two-column">
        <div class="panel">
          <p class="eyebrow">Revenue Trend</p>
          <h2>30-day movement</h2>

          <div class="revenue-row">
            <div>
              <strong>Activation fees</strong>
              <p>{{ formatMoney(dashboard.revenue.activation_fees_30d) }}</p>
            </div>
            <span :class="activationFeeChange >= 0 ? 'positive' : 'negative'">
              {{ activationFeeChange >= 0 ? '+' : ''
              }}{{ activationFeeChange }}%
            </span>
          </div>

          <div class="revenue-row">
            <div>
              <strong>Transactions</strong>
              <p>{{ formatMoney(dashboard.revenue.transactions_30d) }}</p>
            </div>
            <span :class="transactionChange >= 0 ? 'positive' : 'negative'">
              {{ transactionChange >= 0 ? '+' : '' }}{{ transactionChange }}%
            </span>
          </div>
        </div>

        <div class="panel">
          <p class="eyebrow">Platform Health Checks</p>
          <h2>Service readiness</h2>

          <div
            v-for="(value, key) in dashboard.health"
            :key="key"
            class="health-row"
          >
            <span>{{ String(key).replace(/_/g, ' ') }}</span>
            <strong :class="statusClass(value)">{{ value }}</strong>
          </div>
        </div>
      </section>

      <section class="panel">
        <p class="eyebrow">New User Activity Feed</p>
        <h2>Platform audit stream</h2>

        <div v-if="!dashboard.recent_activity.length" class="empty-state">
          No recent platform activity.
        </div>

        <div
          v-for="event in dashboard.recent_activity"
          :key="event.id"
          class="activity-row"
        >
          <div>
            <strong>{{ formatEventName(event.event_type) }}</strong>
            <p>
              <span v-if="event.actor_name">{{ event.actor_name }}</span>
              <span v-else-if="event.actor_email">{{ event.actor_email }}</span>
              <span v-else>System</span>
              · {{ event.entity_type }} #{{ event.entity_id }}
              <span v-if="event.organization_name">
                · {{ event.organization_name }}
              </span>
            </p>
          </div>

          <time>{{ event.created_at }}</time>
        </div>
      </section>

      <section class="panel">
        <p class="eyebrow">Super Admin Actions</p>
        <h2>Command center</h2>

        <div class="action-grid">
          <NuxtLink
            v-for="action in quickActions"
            :key="action.to"
            :to="action.to"
            class="action-card"
          >
            <strong>{{ action.label }}</strong>
            <p>{{ action.description }}</p>
          </NuxtLink>
        </div>
      </section>
    </template>
  </DashboardShell>
</template>

<style scoped>
.search-panel,
.panel,
.stat-card,
.health-card {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 20px;
  background: linear-gradient(
    180deg,
    rgba(15, 23, 42, 0.96),
    rgba(2, 12, 23, 0.96)
  );
  box-shadow: 0 18px 50px rgba(0, 0, 0, 0.28);
  color: #e5eefc;
  padding: 24px;
}

.search-panel {
  margin-bottom: 22px;
}

.search-input {
  width: 100%;
  border: 1px solid rgba(45, 212, 191, 0.28);
  border-radius: 14px;
  background: rgba(2, 12, 23, 0.9);
  color: #f8fafc;
  margin-top: 12px;
  padding: 14px 16px;
}

.search-results {
  display: grid;
  gap: 14px;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  margin-top: 16px;
}

.search-group p {
  color: #67e8f9;
  font-size: 0.75rem;
  font-weight: 900;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

.search-result {
  border: 1px solid rgba(45, 212, 191, 0.16);
  border-radius: 12px;
  color: #e5eefc;
  display: block;
  margin-top: 8px;
  padding: 12px;
  text-decoration: none;
}

.search-result span,
.search-empty {
  color: #a8bdd2;
  display: block;
  margin-top: 4px;
}

.error-panel {
  border: 1px solid rgba(248, 113, 113, 0.38);
  border-radius: 18px;
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
  padding: 18px;
}

.hero-grid,
.metric-grid,
.two-column,
.action-grid,
.totals-grid {
  display: grid;
  gap: 18px;
  margin-bottom: 22px;
}

.hero-grid,
.metric-grid {
  grid-template-columns: repeat(auto-fit, minmax(210px, 1fr));
}

.two-column {
  grid-template-columns: minmax(0, 1.1fr) minmax(320px, 0.9fr);
}

.totals-grid {
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  margin-bottom: 0;
}

.health-card {
  background: radial-gradient(
      circle at top right,
      rgba(52, 211, 153, 0.22),
      transparent 34%
    ),
    linear-gradient(180deg, rgba(15, 23, 42, 0.98), rgba(2, 12, 23, 0.98));
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.12em;
  margin: 0 0 10px;
  text-transform: uppercase;
}

h2 {
  color: #f8fafc;
  font-size: 1.35rem;
  margin: 0 0 14px;
}

.stat-card h2 {
  font-size: 2.4rem;
  margin-bottom: 6px;
}

.health-score {
  color: #6ee7b7;
  font-size: 3.3rem;
  font-weight: 900;
  line-height: 1;
}

.health-label {
  color: #f8fafc;
  font-size: 1.1rem;
  font-weight: 800;
  margin: 10px 0;
}

.muted,
.panel p,
.action-card p,
.alert-row p,
.activity-row p,
.org-row p,
.revenue-row p {
  color: #a8bdd2;
  line-height: 1.55;
}

.section-heading {
  align-items: start;
  display: flex;
  justify-content: space-between;
  gap: 16px;
}

.ghost-button {
  border: 1px solid rgba(45, 212, 191, 0.28);
  border-radius: 999px;
  background: rgba(8, 31, 42, 0.8);
  color: #dff7ff;
  cursor: pointer;
  font-weight: 800;
  padding: 9px 14px;
}

.alert-row,
.activity-row,
.funnel-row,
.status-row,
.action-card,
.org-row,
.revenue-row,
.health-row,
.mini-stat {
  border: 1px solid rgba(45, 212, 191, 0.16);
  border-radius: 14px;
  background: rgba(8, 31, 42, 0.74);
  margin-top: 12px;
  padding: 14px;
}

.alert-row,
.action-card,
.org-row {
  color: #e5eefc;
  display: block;
  text-decoration: none;
}

.alert-row,
.activity-row,
.funnel-row,
.status-row,
.org-row,
.revenue-row,
.health-row,
.mini-stat {
  align-items: center;
  display: flex;
  justify-content: space-between;
  gap: 16px;
}

.alert-row span {
  border-radius: 999px;
  background: rgba(251, 191, 36, 0.14);
  color: #fde68a;
  font-weight: 900;
  min-width: 44px;
  padding: 8px 10px;
  text-align: center;
}

.activity-row time {
  color: #93c5fd;
  font-size: 0.78rem;
  white-space: nowrap;
}

.funnel-row strong,
.status-row strong,
.mini-stat strong {
  color: #6ee7b7;
  font-size: 1.2rem;
}

.funnel-row small {
  color: #93c5fd;
  font-size: 0.78rem;
  margin-left: 6px;
}

.mini-stat span,
.status-row span,
.health-row span {
  color: #a8bdd2;
  text-transform: capitalize;
}

.action-grid {
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  margin-bottom: 0;
}

.action-card:hover,
.alert-row:hover,
.org-row:hover,
.search-result:hover {
  border-color: rgba(96, 165, 250, 0.5);
  transform: translateY(-1px);
}

.empty-state {
  border: 1px dashed rgba(45, 212, 191, 0.28);
  border-radius: 14px;
  color: #cbd5e1;
  padding: 18px;
}

.positive,
.good {
  color: #6ee7b7;
}

.negative,
.bad {
  color: #fca5a5;
}

.watch {
  color: #fde68a;
}

@media (max-width: 980px) {
  .two-column {
    grid-template-columns: 1fr;
  }
}
</style>
