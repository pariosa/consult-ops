<!-- invoice card -->
<template>
  <div class="invoice-card" :class="cardClass">
    <div class="header">
      <h3>Invoice #{{ invoice.id }}</h3>
      <span class="status">{{ invoice.status }}</span>
    </div>

    <div class="grid">
      <div class="row">
        <h3>{{ invoice?.invoice_number || invoice?.title || 'Invoice' }}</h3>

        <p>
          Amount
          <strong>{{ amount.toFixed(2) }}</strong>
        </p>
      </div>

      <div class="row">
        <span>Due</span>
        <strong>{{ invoice.due_date }}</strong>
      </div>

      <div class="row">
        <span>Subtotal</span>
        <strong>{{ formatCurrency(invoice.subtotal) }}</strong>
      </div>

      <div class="row">
        <span>Tax</span>
        <strong>{{ formatCurrency(invoice.tax) }}</strong>
      </div>

      <div class="row total">
        <span>Total</span>
        <strong>{{ formatCurrency(invoice.total) }}</strong>
      </div>
    </div>

    <div class="footer">
      <small>{{ invoice.notes }}</small>
      <small>{{ invoice.created_at }}</small>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{ invoice: any }>();

const formatCurrency = (amount: number) => `$${amount.toFixed(2)}`;

const amount = computed(() => {
  return Number(
    props.invoice?.amount ??
      props.invoice?.amount_cents / 100 ??
      props.invoice?.total ??
      0,
  );
});
const cardClass = computed(() => {
  if (props.invoice.total > 10000) return 'large';
  if (props.invoice.status === 'Paid') return 'paid';
  if (props.invoice.status === 'Overdue') return 'overdue';
  if (props.invoice.status === 'Draft') return 'draft';
  return 'normal';
});
</script>

<style scoped>
.invoice-card {
  position: relative;
  padding: 1.2rem;
  border-radius: 14px;
  background: white;
  color: #111;
  border: 2px solid transparent;
  transition: all 0.25s ease;
}

/* gradient border layer */
.invoice-card::before {
  content: '';
  position: absolute;
  inset: -2px;
  border-radius: 14px;
  z-index: -1;
  opacity: 0.8;
  transition: all 0.3s ease;
}

/* hover interaction */
.invoice-card:hover::before {
  opacity: 1;
  filter: blur(6px);
}

.invoice-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.12);
}

/* ===== STATE STYLES ===== */

/* Paid → green → blue */
.paid {
  background: #f0fdf4;
}
.paid::before {
  background: linear-gradient(135deg, #22c55e, #3b82f6);
}

/* Overdue → red → orange */
.overdue {
  background: #fef2f2;
}
.overdue::before {
  background: linear-gradient(135deg, #ef4444, #f97316);
}

/* Draft → periwinkle → pale blue */
.draft {
  background: #eef2ff;
  border-style: dashed;
}
.draft::before {
  background: linear-gradient(135deg, #a5b4fc, #bae6fd);
}

/* Normal → green → blue */
.normal {
  background: #eff6ff;
}
.normal::before {
  background: linear-gradient(135deg, #22c55e, #3b82f6);
}

/* Large → purple → green */
.large {
  background: #faf5ff;
}
.large::before {
  background: linear-gradient(135deg, #a855f7, #22c55e);
}

/* layout */

.header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 0.5rem;
}

.status {
  font-size: 0.75rem;
  opacity: 0.7;
}

.grid {
  margin-top: 0.5rem;
}

.row {
  display: flex;
  justify-content: space-between;
  padding: 4px 0;
}

.row.total {
  border-top: 1px solid rgba(0, 0, 0, 0.08);
  margin-top: 6px;
  padding-top: 6px;
  font-weight: 600;
}

.footer {
  margin-top: 0.8rem;
  display: flex;
  justify-content: space-between;
  font-size: 0.7rem;
  opacity: 0.6;
}
</style>
