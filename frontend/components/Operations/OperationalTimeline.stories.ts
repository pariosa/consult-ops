import type { Meta, StoryObj } from '@storybook/vue3';
import OperationalTimeline from './OperationalTimeline.vue';

const meta: Meta<typeof OperationalTimeline> = {
  title: 'Operations/OperationalTimeline',
  component: OperationalTimeline,
  args: {
    events: [
      {
        id: 1,
        organization_id: 1,
        actor_user_id: 2,
        entity_type: 'engagement',
        entity_id: 42,
        event_type: 'EngagementContractSent',
        from_status: 'draft',
        to_status: 'pending_signature',
        metadata: '{}',
        created_at: '2026-05-08 09:12:44',
      },
      {
        id: 2,
        organization_id: 1,
        actor_user_id: 4,
        entity_type: 'engagement',
        entity_id: 42,
        event_type: 'EngagementContractSigned',
        from_status: 'pending_signature',
        to_status: 'awaiting_payment',
        metadata: '{}',
        created_at: '2026-05-08 10:03:18',
      },
      {
        id: 3,
        organization_id: 1,
        actor_user_id: null,
        entity_type: 'engagement',
        entity_id: 42,
        event_type: 'EngagementPaymentReceived',
        from_status: 'awaiting_payment',
        to_status: 'active',
        metadata: '{"stripe_event":"checkout.session.completed"}',
        created_at: '2026-05-08 10:11:02',
      },
      {
        id: 4,
        organization_id: 1,
        actor_user_id: 2,
        entity_type: 'engagement',
        entity_id: 42,
        event_type: 'EngagementActivated',
        from_status: 'awaiting_payment',
        to_status: 'active',
        metadata: '{}',
        created_at: '2026-05-08 10:12:30',
      },
      {
        id: 5,
        organization_id: 1,
        actor_user_id: 2,
        entity_type: 'engagement',
        entity_id: 42,
        event_type: 'EngagementMilestoneSubmitted',
        from_status: 'active',
        to_status: 'milestone_review',
        metadata: '{"milestone_id":7}',
        created_at: '2026-05-09 14:22:10',
      },
      {
        id: 6,
        organization_id: 1,
        actor_user_id: 4,
        entity_type: 'engagement',
        entity_id: 42,
        event_type: 'EngagementMilestoneApproved',
        from_status: 'milestone_review',
        to_status: 'active',
        metadata: '{"milestone_id":7}',
        created_at: '2026-05-09 16:45:51',
      },
    ],
  },
};

export default meta;

type Story = StoryObj<typeof OperationalTimeline>;

export const Default: Story = {};

export const Empty: Story = {
  args: {
    events: [],
  },
};

export const WithRiskEvents: Story = {
  args: {
    events: [
      {
        id: 1,
        organization_id: 1,
        actor_user_id: null,
        entity_type: 'engagement',
        entity_id: 99,
        event_type: 'EngagementOverdue',
        from_status: 'active',
        to_status: 'overdue',
        metadata: '{}',
        created_at: '2026-05-10 08:00:00',
      },
      {
        id: 2,
        organization_id: 1,
        actor_user_id: null,
        entity_type: 'engagement',
        entity_id: 99,
        event_type: 'EngagementSuspended',
        from_status: 'overdue',
        to_status: 'suspended',
        metadata: '{}',
        created_at: '2026-05-12 08:00:00',
      },
      {
        id: 3,
        organization_id: 1,
        actor_user_id: 5,
        entity_type: 'engagement',
        entity_id: 99,
        event_type: 'EngagementDisputed',
        from_status: 'suspended',
        to_status: 'disputed',
        metadata: '{"reason":"scope disagreement"}',
        created_at: '2026-05-13 11:34:00',
      },
    ],
  },
};
