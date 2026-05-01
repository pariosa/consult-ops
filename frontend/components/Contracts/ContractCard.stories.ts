import type { Meta, StoryObj } from '@storybook/vue3-vite';
import ContractCard from './ContractCard.vue';

const meta: Meta<typeof ContractCard> = {
  title: 'ERP/Contracts/ContractCard',
  component: ContractCard,
};

export default meta;
type Story = StoryObj<typeof ContractCard>;

const base = {
  id: 1,
  project_id: 10,
  title: 'Website Redesign Agreement',
  status: 'active',
  signed_at: '2026-03-01',
  start_date: '2026-03-05',
  end_date: '2026-06-01',
  value: 15000,
  currency: '$',
  terms: 'Net 30',
  notes: 'Includes UI/UX and frontend build',
  external_id: 'EXT-123',
  created_at: '2026-02-20',
};

export const Active: Story = {
  args: {
    contract: { ...base, status: 'active' },
  },
};

export const Draft: Story = {
  args: {
    contract: {
      ...base,
      status: 'draft',
      signed_at: null,
      value: null,
    },
  },
};

export const Completed: Story = {
  args: {
    contract: {
      ...base,
      status: 'completed',
    },
  },
};

export const Cancelled: Story = {
  args: {
    contract: {
      ...base,
      status: 'cancelled',
    },
  },
};

export const Minimal: Story = {
  args: {
    contract: {
      id: 2,
      project_id: 11,
      title: 'Basic Consulting Agreement',
      status: 'active',
      created_at: '2026-04-01',
    },
  },
};
