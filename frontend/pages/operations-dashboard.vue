<template>
  <div>
    <h2>Operations Dashboard</h2>
    <div class="kpi-grid">
      <KPITile
        title="Active Projects"
        :value="projects.length"
        :series="[{ name: 'Projects', data: projectsByMonth }]"
        :chartOptions="{ chart: { sparkline: { enabled: true } } }"
      />
      <KPITile
        title="Invoices Pending"
        :value="pendingInvoices"
        :series="[{ name: 'Invoices', data: invoicesByMonth }]"
        :chartOptions="{ chart: { sparkline: { enabled: true } } }"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import KPITile from '~/components/KPITile.vue';

const projects = ref<any[]>([]);
const invoices = ref<any[]>([]);
const projectsByMonth = ref<number[]>([3,4,5,2,6]);
const invoicesByMonth = ref<number[]>([5,2,7,3,4]);
const pendingInvoices = ref<number>(0);

onMounted(async () => {
  projects.value = await (await fetch('http://localhost:8000/api/projects')).json();
  invoices.value = await (await fetch('http://localhost:8000/api/invoices')).json();
  pendingInvoices.value = invoices.value.filter(inv => inv.status === 'pending').length;
});
</script>

<style scoped>
.kpi-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px,1fr)); gap:1rem; }
</style>