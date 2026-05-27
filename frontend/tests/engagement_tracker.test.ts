import { mount } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';
import EngagementTracker from '../components/Engagements/EngagementTracker.vue';

describe('EngagementTracker', () => {
  it('renders tracker steps', () => {
    const wrapper = mount(EngagementTracker, {
      props: {
        status: 'draft',
        platformFeeStatus: 'pending',
        milestones: [],
      },
    });

    expect(wrapper.text()).toContain('Engagement Power Track');
    expect(wrapper.text()).toContain('Created');
    expect(wrapper.text()).toContain('Activated');
    expect(wrapper.text()).toContain('Sent');
    expect(wrapper.text()).toContain('Signed');
    expect(wrapper.text()).toContain('Paid');
    expect(wrapper.text()).toContain('Complete');
  });

  it('shows activated as complete when platform fee is paid', () => {
    const wrapper = mount(EngagementTracker, {
      props: {
        status: 'draft',
        platformFeeStatus: 'paid',
        milestones: [],
      },
    });

    expect(wrapper.text()).toContain('33%');
    expect(wrapper.text()).toContain('2/6 charged');
    expect(wrapper.text()).toContain('Activated');
    expect(wrapper.findAll('.battery-cell.tone-complete')).toHaveLength(2);
  });

  it('marks later steps complete for paid status', () => {
    const wrapper = mount(EngagementTracker, {
      props: {
        status: 'paid',
        platformFeeStatus: 'paid',
        milestones: [],
      },
    });

    const text = wrapper.text();

    expect(text).toContain('Created');
    expect(text).toContain('Activated');
    expect(text).toContain('Sent');
    expect(text).toContain('Signed');
    expect(text).toContain('Paid');
    expect(text).toContain('5/6 charged');
  });
});
