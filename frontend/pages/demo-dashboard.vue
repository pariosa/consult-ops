<template>
  <div>
    <h2>Operations Dashboard</h2>
    <div class="dashboard-grid">
      <ProjectCard v-for="project in projects" :key="project.id" :project="project" />
      <InvoiceCard v-for="invoice in invoices" :key="invoice.id" :invoice="invoice"/>
      <ContractCard v-for="contract in contracts" :key="contract.id" :contract="contract"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import ProjectCard from '~/components/ProjectCard.vue';
import InvoiceCard from '~/components/InvoiceCard.vue';
import ContractCard from '~/components/ContractCard.vue';

const projects = ref([]);
const invoices = ref([]);
const contracts = ref([]);

onMounted(async () => {
  projects.value = await (await fetch('http://localhost:8000/api/projects')).json();
  invoices.value = await (await fetch('http://localhost:8000/api/invoices')).json();
  contracts.value = await (await fetch('http://localhost:8000/api/contracts')).json();
});
</script>

<style scoped>
.dashboard-grid { display: grid; grid-template-columns: repeat(auto-fill,minmax(300px,1fr)); gap:1rem; }
</style>