<script setup lang="ts">
import OrganizationSidebar from '~/layouts/organization.vue';
import { useApi } from '~/composables/useApi';
import { usePermissions } from '~/composables/usePermissions';

const api = useApi();
const { canManageFinance, canManageAgreements, canProcessTransactions, role } =
  usePermissions();

const loading = ref(true);
const error = ref('');
const organization = ref<any>(null);

const cards = computed(() =>
  [
    {
      title: 'Members',
      description: 'View active workspace users and assigned roles.',
      to: '/organization/members',
      show: canManageAgreements.value,
    },
    {
      title: 'Invitations',
      description:
        'Invite contractors, finance admins, clients, and operators.',
      to: '/organization/invitations',
      show: canManageAgreements.value,
    },
    {
      title: 'Operational Finance',
      description: 'Review outstanding obligations, balances, and paid totals.',
      to: '/organization/finance',
      show: canManageFinance.value,
    },
    {
      title: 'Engagements',
      description: 'Manage consulting engagements and workflow state.',
      to: '/engagements',
      show: canProcessTransactions.value || canManageAgreements.value,
    },
    {
      title: 'Projects',
      description: 'Track projects connected to clients and engagements.',
      to: '/projects',
      show: true,
    },
    {
      title: 'Clients',
      description: 'Manage client records and verified client parties.',
      to: '/clients',
      show: true,
    },
    {
      title: 'Transactions',
      description:
        'Open an engagement transaction ledger from an engagement page.',
      to: '/engagements',
      show: canProcessTransactions.value,
    },
    {
      title: 'Agreements',
      description: 'Configure payout rules from an engagement agreement page.',
      to: '/engagements',
      show: canManageAgreements.value,
    },
    {
      title: 'Milestones',
      description: 'Submit, approve, and pay milestone work.',
      to: '/engagements',
      show: canProcessTransactions.value,
    },
  ].filter((card) => card.show),
);

async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    organization.value = await api.get('/api/me/organization');
  } catch (err: any) {
    error.value = err?.message || 'Failed to load organization workspace.';
  } finally {
    loading.value = false;
  }
}

onMounted(refresh);
</script>

<template>
  <DashboardShell
    title="Organization HQ"
    subtitle="Your role-aware workspace command center."
  >
    <section v-if="loading" class="portal-section">
      Loading workspace...
    </section>

    <section v-else-if="error" class="form-error">{{ error }}</section>

    <div v-else class="workspace-layout">
      <OrganizationSidebar />

      <main class="workspace-main">
        <section class="portal-section hero">
          <p class="eyebrow">Organization Workspace</p>
          <h2>{{ organization?.name || 'Your Organization' }}</h2>
          <p>
            You are signed in as <strong>{{ role || 'member' }}</strong
            >. The tools below are filtered by your operational permissions.
          </p>
        </section>

        <section class="card-grid">
          <NuxtLink
            v-for="card in cards"
            :key="card.title"
            :to="card.to"
            class="workspace-card"
          >
            <p class="eyebrow">{{ card.title }}</p>
            <h3>{{ card.title }}</h3>
            <p>{{ card.description }}</p>
          </NuxtLink>
        </section>
      </main>
    </div>
  </DashboardShell>
</template>

<style scoped>
.workspace-layout {
  display: grid;
  gap: 20px;
}

.workspace-main {
  min-width: 0;
}

.portal-section {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 18px;
  background: linear-gradient(
    180deg,
    rgba(15, 23, 42, 0.96),
    rgba(2, 12, 23, 0.96)
  );
  color: #e5eefc;
  padding: 24px;
  margin-bottom: 20px;
}

.hero h2 {
  color: #f8fafc;
  margin: 0 0 10px;
}

.hero p {
  color: #cbd5e1;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.12em;
  margin: 0 0 8px;
  text-transform: uppercase;
}

.card-grid {
  display: grid;
  gap: 14px;
}

.workspace-card {
  border: 1px solid rgba(45, 212, 191, 0.18);
  border-radius: 18px;
  background: rgba(8, 31, 42, 0.86);
  color: #cbd5e1;
  padding: 20px;
  text-decoration: none;
  transition:
    transform 0.16s ease,
    border-color 0.16s ease,
    background 0.16s ease;
}

.workspace-card:hover {
  background: rgba(15, 46, 61, 0.96);
  border-color: rgba(45, 212, 191, 0.38);
  transform: translateY(-2px);
}

.workspace-card h3 {
  color: #f8fafc;
  margin: 0 0 8px;
}

.workspace-card p {
  margin: 0;
}

.form-error {
  border: 1px solid rgba(248, 113, 113, 0.38);
  border-radius: 14px;
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
  padding: 16px;
}

@media (min-width: 980px) {
  .workspace-layout {
    grid-template-columns: 280px minmax(0, 1fr);
    align-items: start;
  }

  .card-grid {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }
}
</style>
