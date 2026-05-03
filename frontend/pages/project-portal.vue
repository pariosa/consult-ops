<script setup lang="ts">
import ProjectCard from '~/components/Project/ProjectCard.vue';
import InvoiceCard from '~/components/Invoices/InvoiceCard.vue';
import PaymentCard from '~/components/Payment/PaymentCard.vue';
import ContractCard from '~/components/Contracts/ContractCard.vue';

const { apiFetch } = useApi();

const { data: me } = await useAsyncData('me', () => apiFetch('/api/me'));

const { data: organization } = await useAsyncData('my-organization', () =>
  apiFetch('/api/me/organization'),
);

const { data: summary } = await useAsyncData('project-summary', () =>
  apiFetch('/api/project-portal/summary'),
);

const { data: projects } = await useAsyncData(
  'organization-projects',
  async () => {
    const org = await apiFetch<any>('/api/me/organization');
    return apiFetch(`/api/organizations/${org.id}/projects`);
  },
);

const { data: clients } = await useAsyncData(
  'organization-clients',
  async () => {
    const org = await apiFetch<any>('/api/me/organization');
    return apiFetch(`/api/organizations/${org.id}/clients`);
  },
);

const { data: contracts } = await useAsyncData(
  'organization-contracts',
  async () => {
    const org = await apiFetch<any>('/api/me/organization');
    return apiFetch(`/api/organizations/${org.id}/contracts`);
  },
);

const { data: invoices } = await useAsyncData(
  'organization-invoices',
  async () => {
    const org = await apiFetch<any>('/api/me/organization');
    return apiFetch(`/api/organizations/${org.id}/invoices`);
  },
);

const { data: payments } = await useAsyncData(
  'organization-payments',
  async () => {
    const org = await apiFetch<any>('/api/me/organization');
    return apiFetch(`/api/organizations/${org.id}/payments`);
  },
);
</script>

<template>
  <DashboardShell
    title="Consultant Project Portal"
    :subtitle="`Workspace: ${organization?.name || 'Loading organization...'}`"
  >
    <section class="portal-hero">
      <div>
        <p class="eyebrow">Logged in as</p>
        <h2>{{ me?.email || '—' }}</h2>
      </div>

      <div>
        <p class="eyebrow">Organization</p>
        <h2>{{ organization?.name || '—' }}</h2>
      </div>
    </section>

    <section class="dashboard-grid">
      <KPITile
        label="Assigned Projects"
        :value="summary?.assigned_projects ?? 0"
      />
      <KPITile label="Active Clients" :value="summary?.active_clients ?? 0" />
      <KPITile
        label="Pending Invoices"
        :value="summary?.pending_invoices ?? 0"
      />
    </section>

    <section class="portal-section">
      <div class="section-header">
        <h2>Projects</h2>
        <p>Organization-scoped delivery work.</p>
      </div>

      <div class="card-grid">
        <ProjectCard
          v-for="project in projects || []"
          :key="project.id"
          :project="project"
        />
      </div>
    </section>

    <section class="portal-section">
      <div class="section-header">
        <h2>Contracts</h2>
        <p>Commercial agreements connected to project delivery.</p>
      </div>

      <div class="card-grid">
        <ContractCard
          v-for="contract in contracts || []"
          :key="contract.id"
          :contract="contract"
        />
      </div>
    </section>

    <section class="portal-section">
      <div class="section-header">
        <h2>Invoices</h2>
        <p>Billing status and upcoming receivables.</p>
      </div>

      <div class="card-grid">
        <InvoiceCard
          v-for="invoice in invoices || []"
          :key="invoice.id"
          :invoice="invoice"
        />
      </div>
    </section>

    <section class="portal-section">
      <div class="section-header">
        <h2>Payments</h2>
        <p>Collected payments and reconciliation references.</p>
      </div>

      <div class="card-grid">
        <PaymentCard
          v-for="payment in payments || []"
          :key="payment.id"
          :payment="payment"
        />
      </div>
    </section>
  </DashboardShell>
</template>

<style scoped>
.portal-hero {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.portal-hero > div,
.portal-section {
  border: 1px solid rgba(85, 214, 190, 0.25);
  border-radius: 18px;
  background: rgba(8, 18, 31, 0.84);
  padding: 1.25rem;
  color: #e5f6ff;
}

.eyebrow {
  margin: 0 0 0.35rem;
  color: #55d6be;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  font-size: 0.75rem;
}

.portal-hero h2 {
  margin: 0;
  color: #fff;
}

.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.portal-section {
  margin-top: 1.5rem;
}

.section-header {
  margin-bottom: 1rem;
}

.section-header h2 {
  margin: 0;
  color: #fff;
}

.section-header p {
  margin: 0.25rem 0 0;
  color: #a8bdd2;
}

.card-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 1rem;
}
</style>
