import type { Meta, StoryObj } from '@storybook/vue3';
import EngagementTracker from './EngagementTracker.vue';

const meta: Meta<typeof EngagementTracker> = {
  title: 'Engagements/EngagementTracker',
  component: EngagementTracker,
  tags: ['autodocs'],
  argTypes: {
    status: {
      control: 'select',
      options: [
        'draft',
        'active',
        'contract_sent',
        'contract_signed',
        'work_in_progress',
        'awaiting_review',
        'paid',
        'completed',
      ],
    },
    platformFeeStatus: {
      control: 'select',
      options: ['pending', 'paid', 'waived', 'failed'],
    },
  },
};

export default meta;

type Story = StoryObj<typeof EngagementTracker>;

export const Draft: Story = {
  args: {
    status: 'draft',
    platformFeeStatus: 'pending',
  },
};

export const Activated: Story = {
  args: {
    status: 'active',
    platformFeeStatus: 'paid',
  },
};

export const ContractSent: Story = {
  args: {
    status: 'contract_sent',
    platformFeeStatus: 'paid',
  },
};

export const Signed: Story = {
  args: {
    status: 'contract_signed',
    platformFeeStatus: 'paid',
  },
};

export const Paid: Story = {
  args: {
    status: 'paid',
    platformFeeStatus: 'paid',
  },
};
