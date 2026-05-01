import { mount } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';
import LoginForm from '../components/LoginForm.vue';

describe('LoginForm', () => {
  it('renders consultant login content', () => {
    const wrapper = mount(LoginForm, {
      props: {
        userType: 'consultant',
        submitText: 'Enter Workspace',
      },
    });

    expect(wrapper.text()).toContain('Consultant Workspace Login');
    expect(wrapper.text()).toContain('Enter Workspace');
  });

  it('emits submit payload', async () => {
    const wrapper = mount(LoginForm, {
      props: {
        userType: 'client',
      },
    });

    await wrapper.find('input[type="email"]').setValue('client@example.com');
    await wrapper.find('input[type="password"]').setValue('StrongPass123!');
    await wrapper.find('form').trigger('submit.prevent');

    expect(wrapper.emitted('submit')).toBeTruthy();
    expect(wrapper.emitted('submit')?.[0]).toEqual([
      {
        email: 'client@example.com',
        password: 'StrongPass123!',
        userType: 'client',
      },
    ]);
  });
});
