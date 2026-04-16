<template>
  <div class="project-card" :class="cardClass">
    <div class="header">
      <div>
        <h3>{{ project.name }}</h3>
        <small>Client #{{ project.client_id }}</small>
      </div>
    </div>

    <div class="grid">
      <div class="row" v-if="project.start_date">
        <span>Start</span>
        <strong>{{ project.start_date }}</strong>
      </div>

      <div class="row" v-if="project.end_date">
        <span>End</span>
        <strong>{{ project.end_date }}</strong>
      </div>
    </div>

    <div class="footer">
      <small>{{ project.description }}</small>
      <small>{{ project.created_at }}</small>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

type Project = {
  id: number;
  client_id: number;
  name: string;
  start_date?: string | null;
  end_date?: string | null;
  description?: string | null;
  created_at: string;
};

const props = defineProps<{ project: Project }>();

const cardClass = computed(() => {
  if (!props.project.start_date) return 'draft';
  if (props.project.end_date) return 'completed';
  return 'active';
});
</script>

<style scoped>
.project-card {
  position: relative;
  padding: 1.2rem;
  border-radius: 14px;
  background: white;
  color: #111;
  border: 2px solid transparent;
  transition: all 0.25s ease;
}

.project-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.12);
}

.project-card::before {
  content: '';
  position: absolute;
  inset: -2px;
  border-radius: 14px;
  z-index: -1;
  opacity: 0.85;
}

.project-card:hover::before {
  filter: blur(6px);
}

/* STATES */

.active {
  background: #f0fdf4;
}
.active::before {
  background: linear-gradient(135deg, #22c55e, #3b82f6);
}

.draft {
  background: #eef2ff;
  border-style: dashed;
}
.draft::before {
  background: linear-gradient(135deg, #a5b4fc, #bae6fd);
}

.completed {
  background: #f1f5f9;
}
.completed::before {
  background: linear-gradient(135deg, #94a3b8, #60a5fa);
}

/* layout */

.header {
  margin-bottom: 0.5rem;
}

.row {
  display: flex;
  justify-content: space-between;
  padding: 4px 0;
}

.footer {
  margin-top: 0.8rem;
  display: flex;
  justify-content: space-between;
  font-size: 0.7rem;
  opacity: 0.6;
}
</style>
