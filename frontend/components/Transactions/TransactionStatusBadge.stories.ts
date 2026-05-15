import type { Meta, StoryObj } from '@storybook/vue3';
import TransactionStatusBadge from './TransactionStatusBadge.vue';

const meta: Meta<typeof TransactionStatusBadge> = {
  title: 'Transactions/TransactionStatusBadge',
  component: TransactionStatusBadge,
};

export default meta;

type Story = StoryObj<typeof TransactionStatusBadge>;

export const Pending: Story = {
  args: { status: 'pending' },
};

export const Processing: Story = {
  args: { status: 'processing' },
};

export const Paid: Story = {
  args: { status: 'paid' },
};

export const Failed: Story = {
  args: { status: 'failed' },
};
