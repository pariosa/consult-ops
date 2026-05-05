import { mount } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';
import EngagementTracker from '../components/Engagements/EngagementTracker.vue';

describe('EngagementTracker', () => {
  it('renders tracker steps', () => {
    const wrapper = mount(EngagementTracker, {
      props: {
        status: 'draft',
        platformFeeStatus: 'pending',
      },
    });

    expect(wrapper.text()).toContain('Engagement Created');
    expect(wrapper.text()).toContain('Activated');
    expect(wrapper.text()).toContain('Contract Sent');
    expect(wrapper.text()).toContain('Signed');
    expect(wrapper.text()).toContain('Work Review');
    expect(wrapper.text()).toContain('Paid');
  });

  it('shows activated as complete when platform fee is paid', () => {
    const wrapper = mount(EngagementTracker, {
      props: {
        status: 'active',
        platformFeeStatus: 'paid',
      },
    });

    expect(wrapper.text()).toContain('✓');
    expect(wrapper.text()).toContain('Activated');
  });

  it('marks later steps complete for paid status', () => {
    const wrapper = mount(EngagementTracker, {
      props: {
        status: 'paid',
        platformFeeStatus: 'paid',
      },
    });

    const text = wrapper.text();

    expect(text).toContain('Engagement Created');
    expect(text).toContain('Activated');
    expect(text).toContain('Contract Sent');
    expect(text).toContain('Signed');
    expect(text).toContain('Work Review');
    expect(text).toContain('Paid');
  });
});
