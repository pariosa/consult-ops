<template>
  <div class="contract-card" :class="cardClass">
    <div class="header">
      <div>
        <h3>{{ contract.title }}</h3>
        <small>ID: {{ contract.id }}</small>
      </div>
      <span class="status">{{ contract.status }}</span>
    </div>

    <div class="grid">
      <div class="row" v-if="contract.value">
        <span>Value</span>
        <strong>{{ formatCurrency(contract.value, contract.currency) }}</strong>
      </div>

      <div class="row" v-if="contract.start_date">
        <span>Start</span>
        <strong>{{ contract.start_date }}</strong>
      </div>

      <div class="row" v-if="contract.end_date">
        <span>End</span>
        <strong>{{ contract.end_date }}</strong>
      </div>

      <div class="row" v-if="contract.signed_at">
        <span>Signed</span>
        <strong>{{ contract.signed_at }}</strong>
      </div>
    </div>

    <div class="footer">
      <small v-if="contract.external_id">Ext: {{ contract.external_id }}</small>
      <small>{{ contract.created_at }}</small>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

type Contract = {
  id: number;
  project_id: number;
  title: string;
  status: string;
  signed_at?: string | null;
  start_date?: string | null;
  end_date?: string | null;
  value?: number | null;
  currency?: string | null;
  terms?: string | null;
  notes?: string | null;
  external_id?: string | null;
  created_at: string;
};

const props = defineProps<{ contract: Contract }>();

const formatCurrency = (amount?: number | null, currency?: string | null) => {
  if (!amount) return '-';
  return `${currency || '$'}${amount.toFixed(2)}`;
};

const cardClass = computed(() => {
  const s = props.contract.status?.toLowerCase();

  if (s === 'active') return 'active';
  if (s === 'draft') return 'draft';
  if (s === 'completed') return 'completed';
  if (s === 'cancelled') return 'cancelled';

  return 'normal';
});
</script>

<style scoped>
.contract-card {
  position: relative;
  padding: 1.2rem;
  border-radius: 14px;
  background: white;
  color: #111;
  border: 2px solid transparent;
  transition: all 0.25s ease;
}

.contract-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.12);
}

/* gradient border */
.contract-card::before {
  content: '';
  position: absolute;
  inset: -2px;
  border-radius: 14px;
  z-index: -1;
  opacity: 0.85;
  transition: all 0.3s ease;
}

.contract-card:hover::before {
  opacity: 1;
  filter: blur(6px);
}

/* ===== STATES ===== */

/* active → green → blue */
.active {
  background: #f0fdf4;
}
.active::before {
  background: linear-gradient(135deg, #22c55e, #3b82f6);
}

/* draft → periwinkle */
.draft {
  background: #eef2ff;
  border-style: dashed;
}
.draft::before {
  background: linear-gradient(135deg, #a5b4fc, #bae6fd);
}

/* completed → neutral calm */
.completed {
  background: #f1f5f9;
}
.completed::before {
  background: linear-gradient(135deg, #94a3b8, #60a5fa);
}

/* cancelled → red/orange */
.cancelled {
  background: #fef2f2;
}
.cancelled::before {
  background: linear-gradient(135deg, #ef4444, #f97316);
}

/* fallback */
.normal {
  background: #eff6ff;
}
.normal::before {
  background: linear-gradient(135deg, #22c55e, #3b82f6);
}

/* layout */

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.status {
  font-size: 0.75rem;
  opacity: 0.7;
  text-transform: capitalize;
}

.grid {
  margin-top: 0.6rem;
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
