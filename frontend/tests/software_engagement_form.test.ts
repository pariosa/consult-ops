import { mount } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';
import SoftwareEngagementForm from '../components/SoftwareEngagementForm.vue';

describe('SoftwareEngagementForm', () => {
  it('renders software engagement fields', () => {
    const wrapper = mount(SoftwareEngagementForm, {
      props: {
        projectId: 1,
        organizationId: 1,
        mockSubmit: true,
      },
    });

    expect(wrapper.text()).toContain('Contractor Name');
    expect(wrapper.text()).toContain('Contractor Email');
    expect(wrapper.text()).toContain('Role');
    expect(wrapper.text()).toContain('Engagement Title');
    expect(wrapper.text()).toContain('Scope of Work');
    expect(wrapper.text()).toContain('Deliverables');
    expect(wrapper.text()).toContain('Repo URL');
  });

  it('emits created event on mock submit', async () => {
    const wrapper = mount(SoftwareEngagementForm, {
      props: {
        projectId: 1,
        organizationId: 1,
        mockSubmit: true,
      },
    });

    await wrapper.find('input').setValue('Peter Dev');

    const inputs = wrapper.findAll('input');
    await inputs[1].setValue('peter@example.com');
    await inputs[2].setValue('Build Client Portal MVP');

    const textareas = wrapper.findAll('textarea');
    await textareas[0].setValue(
      'Build auth, dashboard, milestones, and billing.',
    );
    await textareas[1].setValue('Rust API, Nuxt frontend, deployment.');

    await inputs[3].setValue('https://github.com/example/repo');
    await inputs[4].setValue('200000');

    await wrapper.find('form').trigger('submit.prevent');

    const emitted = wrapper.emitted('created');

    expect(emitted).toBeTruthy();
    expect(emitted?.[0][0]).toMatchObject({
      contractor_name: 'Peter Dev',
      contractor_email: 'peter@example.com',
      title: 'Build Client Portal MVP',
      status: 'draft',
      platform_fee_status: 'pending',
    });
  });
});
