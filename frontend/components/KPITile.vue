<template>
  <div class="kpi-tile">
    <h3>{{ title }}</h3>
    <p class="value">{{ value }}</p>
    <apexchart type="line" height="100" :options="chartOptions" :series="series"/>
  </div>
</template>

<script setup lang="ts">
import ApexCharts from 'vue3-apexcharts'
defineProps<{
  title: string,
  value: string | number,
  series: Array<{ name: string, data: number[] }>,
  chartOptions: object
}>()
</script>

<style scoped>
.kpi-tile { border: 1px solid #ccc; border-radius: 10px; padding: 1rem; background:#fdfdfd; margin: 0.5rem; }
.value { font-size: 2rem; font-weight: bold; }
</style>