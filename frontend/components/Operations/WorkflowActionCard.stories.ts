import type { Meta, StoryObj } from '@storybook/vue3';
import WorkflowActionCard from './WorkflowActionCard.vue';

const meta: Meta<typeof WorkflowActionCard> = {
  title: 'Operations/WorkflowActionCard',
  component: WorkflowActionCard,
  tags: ['autodocs'],
};

export default meta;

type Story = StoryObj<typeof WorkflowActionCard>;

export const ApprovalRequired: Story = {
  args: {
    title: 'Milestone Awaiting Approval',
    description:
      'Operational timeline implementation submitted and waiting for review.',
    status: 'Needs Approval',
    severity: 'warning',
    primaryLabel: 'Approve',
    secondaryLabel: 'Review',
  },
};

export const PaymentPending: Story = {
  args: {
    title: 'Activation Fee Pending',
    description: 'Client has not completed engagement activation payment.',
    status: 'Awaiting Payment',
    severity: 'critical',
    primaryLabel: 'Send Reminder',
    secondaryLabel: 'Open Billing',
  },
};

export const SuccessState: Story = {
  args: {
    title: 'Milestone Approved',
    description: 'Client approved deliverables and payment was released.',
    status: 'Completed',
    severity: 'success',
    primaryLabel: 'Open Engagement',
  },
};

export const InfoState: Story = {
  args: {
    title: 'Contract Sent',
    description:
      'Software contract has been generated and emailed to contractor.',
    status: 'Awaiting Signature',
    severity: 'info',
    primaryLabel: 'View Contract',
    secondaryLabel: 'Resend',
  },
};
