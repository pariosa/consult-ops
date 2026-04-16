import type { Meta, StoryObj } from '@storybook/vue3';
import PaymentCard from './PaymentCard.vue';

const meta: Meta<typeof PaymentCard> = {
  title: 'ERP/Payments/PaymentCard',
  component: PaymentCard,
};

export default meta;
type Story = StoryObj<typeof PaymentCard>;

const base = {
  id: 1,
  invoice_id: 1001,
  paid_at: '2026-04-01',
  amount: 1200,
  currency: '$',
  method: 'card',
  reference: 'txn_123',
  notes: 'Stripe payment',
  created_at: '2026-04-01',
};

export const CardPayment: Story = {
  args: { payment: { ...base } },
};

export const BankTransfer: Story = {
  args: {
    payment: { ...base, method: 'bank_transfer' },
  },
};

export const Cash: Story = {
  args: {
    payment: { ...base, method: 'cash' },
  },
};

export const Pending: Story = {
  args: {
    payment: { ...base, paid_at: null },
  },
};
