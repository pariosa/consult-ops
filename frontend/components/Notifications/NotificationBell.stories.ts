import type { Meta, StoryObj } from '@storybook/vue3';
import NotificationBell from './NotificationBell.vue';

const notifications = [
  {
    id: 1,
    title: 'Milestone Approved',
    message: 'Milestone #4 was approved for Atlas Field Consulting.',
    notification_type: 'milestone_approved',
    read_at: null,
    created_at: '2026-05-26 09:14:00',
  },
  {
    id: 2,
    title: 'Payout Released',
    message: 'A contractor payout of $2,400 has been marked paid.',
    notification_type: 'transaction_paid',
    read_at: null,
    created_at: '2026-05-26 10:45:00',
  },
  {
    id: 3,
    title: 'Organization Invitation',
    message: 'You were invited to join Verdant Retail Systems.',
    notification_type: 'organization_invitation',
    read_at: '2026-05-26 11:00:00',
    created_at: '2026-05-26 10:58:00',
  },
];

const meta: Meta<typeof NotificationBell> = {
  title: 'Notifications/NotificationBell',
  component: NotificationBell,
  parameters: {
    layout: 'fullscreen',
    backgrounds: {
      default: 'dark',
    },
  },
  args: {
    notifications,
    loading: false,
  },
};

export default meta;

type Story = StoryObj<typeof NotificationBell>;

export const Default: Story = {};

export const Empty: Story = {
  args: {
    notifications: [],
  },
};

export const Loading: Story = {
  args: {
    notifications: [],
    loading: true,
  },
};

export const ManyUnread: Story = {
  args: {
    notifications: [
      {
        id: 1,
        title: 'Engagement Activated',
        message: 'An engagement has moved into active status.',
        notification_type: 'engagement_activated',
        read_at: null,
        created_at: '2026-05-26 08:00:00',
      },
      {
        id: 2,
        title: 'Milestone Submitted',
        message: 'A contractor submitted milestone deliverables.',
        notification_type: 'milestone_submitted',
        read_at: null,
        created_at: '2026-05-26 08:30:00',
      },
      {
        id: 3,
        title: 'Invoice Generated',
        message: 'Invoice #2041 was generated for review.',
        notification_type: 'invoice_created',
        read_at: null,
        created_at: '2026-05-26 09:00:00',
      },
      {
        id: 4,
        title: 'Operational Risk',
        message: 'A transaction has entered disputed state.',
        notification_type: 'transaction_disputed',
        read_at: null,
        created_at: '2026-05-26 09:12:00',
      },
    ],
  },
};

export const MixedReadState: Story = {
  args: {
    notifications: [
      {
        id: 1,
        title: 'Contract Signed',
        message: 'The consulting agreement was signed.',
        notification_type: 'contract_signed',
        read_at: '2026-05-26 09:20:00',
        created_at: '2026-05-26 09:00:00',
      },
      {
        id: 2,
        title: 'Payment Received',
        message: 'Stripe confirmed payment settlement.',
        notification_type: 'payment_received',
        read_at: null,
        created_at: '2026-05-26 09:45:00',
      },
    ],
  },
};

export const InHeader: Story = {
  render: (args) => ({
    components: { NotificationBell },
    setup() {
      return { args };
    },
    template: `
      <div
        style="
          background:#020617;
          min-height:100vh;
          padding:40px;
          display:flex;
          justify-content:flex-end;
          align-items:flex-start;
        "
      >
        <NotificationBell v-bind="args" />
      </div>
    `,
  }),
};
