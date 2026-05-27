export type EngagementNextAction = {
  tone: 'setup' | 'waiting' | 'success' | 'risk' | 'finance';
  label: string;
  title: string;
  description: string;
  primaryLabel?: string;
  primaryTo?: string;
  secondaryLabel?: string;
  secondaryTo?: string;
};

export function getEngagementNextAction(engagement: any): EngagementNextAction {
  const status = engagement?.status || 'draft';
  const billingStatus = engagement?.platform_fee_status;
  if (status === 'completed') {
    return {
      tone: 'success',
      label: 'Completed',
      title: 'This engagement is fully closed out.',
      description:
        'All milestones have been completed and the final sign-off has been recorded.',
      primaryLabel: 'Review Timeline',
      primaryTo: `/engagements/${engagement.id}`,
      secondaryLabel: 'View Transactions',
      secondaryTo: `/engagements/${engagement.id}/transactions`,
    };
  }
  if (status === 'draft') {
    return {
      tone: 'setup',
      label: 'Setup Required',
      title: 'Prepare this engagement for agreement review.',
      description:
        'The engagement exists, but it is not yet ready for client commitment. Finalize scope, deliverables, and agreement details before sending it forward.',
      primaryLabel: 'Generate Agreement',
      primaryTo: `/engagements/${engagement.id}/software-contract`,
      secondaryLabel: 'Edit Engagement',
      secondaryTo: `/engagements/${engagement.id}`,
    };
  }

  if (status === 'pending_signature') {
    return {
      tone: 'waiting',
      label: 'Awaiting Signature',
      title: 'The next checkpoint is agreement signature.',
      description:
        'The contract has been sent. Keep the engagement in this state until the responsible party signs or confirms acceptance.',
      primaryLabel: 'View Timeline',
      primaryTo: `/engagements/${engagement.id}`,
    };
  }

  if (status === 'awaiting_payment' || billingStatus === 'pending') {
    return {
      tone: 'finance',
      label: 'Activation Pending',
      title: 'Activation payment is the next operational gate.',
      description:
        'This engagement should not begin active work until the activation fee is paid or billing is confirmed.',
      primaryLabel: 'Open Billing',
      primaryTo: `/engagements/${engagement.id}/billing`,
      secondaryLabel: 'View Engagement',
      secondaryTo: `/engagements/${engagement.id}`,
    };
  }

  if (status === 'active') {
    return {
      tone: 'success',
      label: 'Active',
      title: 'Work can move through milestone delivery.',
      description:
        'The engagement is active. The next operational step is creating, submitting, or approving milestones so payment obligations can be tracked.',
      primaryLabel: 'Manage Milestones',
      primaryTo: `/engagements/${engagement.id}/milestones`,
      secondaryLabel: 'View Transactions',
      secondaryTo: `/engagements/${engagement.id}/transactions`,
    };
  }

  if (status === 'milestone_review') {
    return {
      tone: 'waiting',
      label: 'Client Review',
      title: 'A milestone is waiting for approval.',
      description:
        'Review submitted deliverables and approve the milestone when it satisfies the agreement. Approval may generate payout obligations.',
      primaryLabel: 'Review Milestones',
      primaryTo: `/engagements/${engagement.id}/milestones`,
      secondaryLabel: 'View Timeline',
      secondaryTo: `/engagements/${engagement.id}`,
    };
  }

  if (status === 'completed') {
    return {
      tone: 'success',
      label: 'Completed',
      title: 'This engagement is complete.',
      description:
        'The core workflow is finished. Use the timeline and transaction history as the operational audit trail.',
      primaryLabel: 'Review Timeline',
      primaryTo: `/engagements/${engagement.id}`,
      secondaryLabel: 'View Transactions',
      secondaryTo: `/engagements/${engagement.id}/transactions`,
    };
  }

  if (
    status === 'cancelled' ||
    status === 'disputed' ||
    status === 'suspended'
  ) {
    return {
      tone: 'risk',
      label: 'Attention Required',
      title: 'This engagement needs administrative review.',
      description:
        'The engagement is not in a normal working state. Review the timeline, contract status, and payment obligations before taking further action.',
      primaryLabel: 'Review Timeline',
      primaryTo: `/engagements/${engagement.id}`,
      secondaryLabel: 'View Transactions',
      secondaryTo: `/engagements/${engagement.id}/transactions`,
    };
  }

  return {
    tone: 'setup',
    label: 'Next Step',
    title: 'Review this engagement before continuing.',
    description:
      'Consult Ops could not determine a specific next action. Review the engagement status, timeline, and billing state.',
    primaryLabel: 'View Engagement',
    primaryTo: `/engagements/${engagement.id}`,
  };
}
