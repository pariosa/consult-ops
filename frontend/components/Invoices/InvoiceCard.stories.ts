import type { Meta, StoryObj } from '@storybook/vue3';
import InvoiceCard from './InvoiceCard.vue';

const meta: Meta<typeof InvoiceCard> = {
  title: 'ERP/Invoices/InvoiceCard',
  component: InvoiceCard,
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof InvoiceCard>;

const base = {
  id: 'INV-1001',
  amount: 1200,
  status: 'Paid',
  due_date: '2026-05-01',
  subtotal: 1000,
  tax: 200,
  total: 1200,
  currency: 'USD',
  notes: 'Monthly consulting',
  created_at: '2026-04-01',
};

export const Paid: Story = {
  args: {
    invoice: {
      ...base,
      status: 'Paid',
    },
  },
};

export const Overdue: Story = {
  args: {
    invoice: {
      ...base,
      id: 'INV-1002',
      status: 'Overdue',
      due_date: '2026-03-01',
    },
  },
};

export const Draft: Story = {
  args: {
    invoice: {
      ...base,
      id: 'INV-1003',
      status: 'Draft',
      total: 0,
      subtotal: 0,
      tax: 0,
    },
  },
};

export const Normal: Story = {
  args: {
    invoice: {
      ...base,
      id: 'INV-1004',
      status: 'Sent',
      total: 3200,
      subtotal: 3000,
      tax: 200,
    },
  },
};

export const LargeInvoice: Story = {
  args: {
    invoice: {
      ...base,
      id: 'INV-9000',
      status: 'Paid',
      total: 25000,
      subtotal: 22000,
      tax: 3000,
      notes: 'Enterprise engagement',
    },
  },
};
