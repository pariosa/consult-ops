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

const projects = ref([]);
const invoices = ref([]);
const contracts = ref([]);
const config = useRuntimeConfig();
const { apiFetch } = useApi();
onMounted(async () => {
  projects.value = await apiFetch(`${config.public.apiBase}/api/projects`);
  invoices.value = await apiFetch(`${config.public.apiBase}/api/invoices`);
  contracts.value = await apiFetch(`${config.public.apiBase}/api/contracts`);
});
</script>

<style scoped>
.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1rem;
}
</style>
