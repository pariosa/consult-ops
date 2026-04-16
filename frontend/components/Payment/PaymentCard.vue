<template>
  <div class="payment-card" :class="cardClass">
    <div class="header">
      <div>
        <h3>Payment #{{ payment.id }}</h3>
        <small>Invoice: {{ payment.invoice_id }}</small>
      </div>
      <span class="method">{{ payment.method || 'unknown' }}</span>
    </div>

    <div class="grid">
      <div class="row">
        <span>Amount</span>
        <strong>{{ formatCurrency(payment.amount, payment.currency) }}</strong>
      </div>

      <div class="row" v-if="payment.paid_at">
        <span>Paid</span>
        <strong>{{ payment.paid_at }}</strong>
      </div>

      <div class="row" v-if="payment.reference">
        <span>Reference</span>
        <strong>{{ payment.reference }}</strong>
      </div>
    </div>

    <div class="footer">
      <small>{{ payment.notes }}</small>
      <small>{{ payment.created_at }}</small>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

type Payment = {
  id: number;
  invoice_id: number;
  paid_at?: string | null;
  amount: number;
  currency?: string | null;
  method?: string | null;
  reference?: string | null;
  notes?: string | null;
  created_at: string;
};

const props = defineProps<{ payment: Payment }>();

const formatCurrency = (amount: number, currency?: string | null) =>
  `${currency || '$'}${amount.toFixed(2)}`;

const cardClass = computed(() => {
  if (!props.payment.paid_at) return 'pending';

  const method = props.payment.method?.toLowerCase();
  if (method === 'card') return 'card';
  if (method === 'bank_transfer') return 'bank';

  return 'paid';
});
</script>

<style scoped>
.payment-card {
  position: relative;
  padding: 1.2rem;
  border-radius: 14px;
  background: white;
  color: #111;
  border: 2px solid transparent;
  transition: all 0.25s ease;
}

.payment-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.12);
}

.payment-card::before {
  content: '';
  position: absolute;
  inset: -2px;
  border-radius: 14px;
  z-index: -1;
  opacity: 0.85;
  transition: all 0.3s ease;
}

.payment-card:hover::before {
  opacity: 1;
  filter: blur(6px);
}

/* STATES */

/* default paid */
.paid {
  background: #f0fdf4;
}
.paid::before {
  background: linear-gradient(135deg, #22c55e, #3b82f6);
}

/* card payments */
.card {
  background: #ecfeff;
}
.card::before {
  background: linear-gradient(135deg, #06b6d4, #3b82f6);
}

/* bank transfer */
.bank {
  background: #f0f9ff;
}
.bank::before {
  background: linear-gradient(135deg, #38bdf8, #6366f1);
}

/* pending */
.pending {
  background: #fef9c3;
}
.pending::before {
  background: linear-gradient(135deg, #facc15, #f97316);
}

/* layout */

.header {
  display: flex;
  justify-content: space-between;
}

.method {
  font-size: 0.75rem;
  opacity: 0.7;
  text-transform: capitalize;
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
