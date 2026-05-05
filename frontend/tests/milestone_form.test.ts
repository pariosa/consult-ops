import { mount } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';
import MilestoneForm from '../components/Engagements/MilestoneForm.vue';

describe('MilestoneForm', () => {
  it('renders milestone fields', () => {
    const wrapper = mount(MilestoneForm, {
      props: {
        engagementId: 1,
        mockSubmit: true,
      },
    });

    expect(wrapper.text()).toContain('Add Milestone');
    expect(wrapper.text()).toContain('Title');
    expect(wrapper.text()).toContain('Description');
    expect(wrapper.text()).toContain('Amount in cents');
    expect(wrapper.text()).toContain('Due Date');
  });

  it('emits created event on mock submit', async () => {
    const wrapper = mount(MilestoneForm, {
      props: {
        engagementId: 1,
        mockSubmit: true,
      },
    });

    const inputs = wrapper.findAll('input');
    const textareas = wrapper.findAll('textarea');

    await inputs[0].setValue('Stripe Billing');
    await textareas[0].setValue('Implement $10 activation checkout.');
    await inputs[1].setValue('50000');
    await inputs[2].setValue('2026-05-15');

    await wrapper.find('form').trigger('submit.prevent');

    const emitted = wrapper.emitted('created');

    expect(emitted).toBeTruthy();
    expect(emitted?.[0][0]).toMatchObject({
      engagement_id: 1,
      title: 'Stripe Billing',
      description: 'Implement $10 activation checkout.',
      amount_cents: 50000,
      due_date: '2026-05-15',
      status: 'pending',
    });
  });
});
