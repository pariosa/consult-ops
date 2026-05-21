import { mount } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';
import SoftwareEngagementForm from '../components/EngagementForms/SoftwareEngagementForm.vue';
import { vi } from 'vitest';
import { nextTick } from 'vue';

vi.stubGlobal('useRouter', () => ({
  push: vi.fn(),
}));
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

  it('emits submit event on form submit', async () => {
    const wrapper = mount(SoftwareEngagementForm, {
      props: {
        projects: [
          {
            id: 1,
            name: 'Client Portal MVP',
          },
        ],
      },
    });

    const inputs = wrapper.findAll('input');
    const textareas = wrapper.findAll('textarea');

    await inputs[0].setValue('Peter Dev');
    await inputs[1].setValue('peter@example.com');
    await inputs[2].setValue('Build Client Portal MVP');

    await textareas[0].setValue(
      'Build auth, dashboard, milestones, and billing.',
    );
    await textareas[1].setValue('Rust API, Nuxt frontend, deployment.');

    await inputs[3].setValue('https://github.com/example/repo');
    await inputs[4].setValue('200000');

    await wrapper.find('form').trigger('submit.prevent');
    await nextTick();

    const emitted = wrapper.emitted('submit');
    expect(emitted).toBeTruthy();

    expect(emitted?.[0][0]).toMatchObject({
      project_id: 1,
      contractor_name: 'Peter Dev',
      contractor_email: 'peter@example.com',
      title: 'Build Client Portal MVP',
      scope_of_work: 'Build auth, dashboard, milestones, and billing.',
      deliverables: 'Rust API, Nuxt frontend, deployment.',
      repo_url: 'https://github.com/example/repo',
      amount_cents: 200000,
      currency: 'usd',
    });
  });
});
