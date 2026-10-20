<template>
  <div>
    <h2>Operations Dashboard</h2>
    <div class="dashboard-grid">
      <ProjectCard
        v-for="project in projects"
        :key="project.id"
        :project="project"
      />
      <InvoiceCard
        v-for="invoice in invoices"
        :key="invoice.id"
        :invoice="invoice"
      />
      <ContractCard
        v-for="contract in contracts"
        :key="contract.id"
        :contract="contract"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import ProjectCard from '~/components/Project/ProjectCard.vue';
import InvoiceCard from '~/components/Invoices/InvoiceCard.vue';
import ContractCard from '~/components/Contracts/ContractCard.vue';
import { useApi } from '~/composables/useApi';
interface DemoProject {
  id: number;
  client_id: number;
  name: string;
  created_at: string;
}

interface DemoContract {
  id: number;
  project_id: number;
  title: string;
  status: string;
  created_at: string;
}

const projects = ref<DemoProject[]>([]);
const contracts = ref<DemoContract[]>([]);
const invoices = ref<Array<{ id: number } & Record<string, unknown>>>([]); 
const config = useRuntimeConfig();
const { apiFetch } = useApi();
onMounted(async () => {
  projects.value = await apiFetch<DemoProject[]>('/api/projects');

  contracts.value = await apiFetch<DemoContract[]>('/api/contracts');

  invoices.value = await apiFetch<
    Array<{ id: number } & Record<string, unknown>>
  >('/api/invoices');

  }
);
</script>

<style scoped>
.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1rem;
}
</style>
