import type { Meta, StoryObj } from '@storybook/vue3';
import OperationalTransactionTable from './OperationalTransactionTable.vue';

const meta: Meta<typeof OperationalTransactionTable> = {
  title: 'Transactions/OperationalTransactionTable',
  component: OperationalTransactionTable,
  args: {
    transactions: [
      {
        id: 1,
        organization_id: 1,
        agreement_id: 1,
        engagement_id: 1,
        milestone_id: 1,
        from_party_id: 1,
        to_party_id: 2,
        transaction_type: 'contractor_payout',
        amount_cents: 250000,
        currency: 'usd',
        status: 'pending',
        trigger_event: 'MilestoneApproved',
        created_at: '2026-05-15 04:56:01',
      },
      {
        id: 2,
        organization_id: 1,
        agreement_id: 1,
        engagement_id: 1,
        milestone_id: 1,
        from_party_id: 2,
        to_party_id: 3,
        transaction_type: 'subcontractor_payout',
        amount_cents: 75000,
        currency: 'usd',
        status: 'processing',
        trigger_event: 'MilestoneApproved',
        created_at: '2026-05-15 04:57:01',
      },
      {
        id: 3,
        organization_id: 1,
        agreement_id: 2,
        engagement_id: 1,
        milestone_id: null,
        from_party_id: 2,
        to_party_id: 1,
        transaction_type: 'dividend',
        amount_cents: 5000,
        currency: 'usd',
        status: 'paid',
        trigger_event: 'EngagementCompleted',
        created_at: '2026-05-15 05:00:01',
      },
    ],
  },
};

export default meta;

type Story = StoryObj<typeof OperationalTransactionTable>;

export const Default: Story = {};

export const Empty: Story = {
  args: {
    transactions: [],
  },
};
