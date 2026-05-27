import { mount } from '@vue/test-utils';
import { describe, expect, it, vi, beforeEach } from 'vitest';
import LoginForm from '../components/LoginForm.vue';
import AppHeader from '../components/AppHeader.vue';
import ForgotPasswordForm from '../components/ForgotPasswordForm.vue';
import ResetPasswordForm from '../components/ResetPasswordForm.vue';
import RegisterForm from '../components/RegisterForm.vue';

vi.stubGlobal('localStorage', {
  getItem: vi.fn(),
  setItem: vi.fn(),
  removeItem: vi.fn(),
  clear: vi.fn(),
});

vi.mock('#app', () => ({
  defineNuxtPlugin: vi.fn(),
  useNuxtApp: vi.fn(() => ({})),
}));

vi.mock('#imports', () => ({
  navigateTo: vi.fn(),
}));

vi.stubGlobal('useRouter', () => ({
  push: vi.fn(),
}));
describe('LoginForm', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders consultant login copy by default', () => {
    const wrapper = mount(LoginForm);

    expect(wrapper.text()).toContain('Consultant Workspace Login');
    expect(wrapper.text()).toContain(
      'Manage clients, projects, invoices, contracts, and payments.',
    );
  });

  it('renders client login copy when userType is client', () => {
    const wrapper = mount(LoginForm, {
      props: {
        userType: 'client',
        submitText: 'Open Client Portal',
      },
    });

    expect(wrapper.text()).toContain('Client Portal Login');
    expect(wrapper.text()).toContain('Open Client Portal');
  });

  it('renders admin login copy when userType is admin', () => {
    const wrapper = mount(LoginForm, {
      props: {
        userType: 'admin',
        submitText: 'Enter Admin Console',
      },
    });

    expect(wrapper.text()).toContain('Admin Console Login');
    expect(wrapper.text()).toContain('Enter Admin Console');
  });

  it('emits submit event with email, password, and userType', async () => {
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
        remember_me: false,
      },
    ]);
  });

  it('shows an error if submitted empty', async () => {
    const wrapper = mount(LoginForm);

    await wrapper.find('form').trigger('submit.prevent');

    expect(wrapper.text()).toContain('Both fields are required');
    expect(wrapper.emitted('submit')).toBeFalsy();
  });
});

describe('ForgotPasswordForm', () => {
  it('renders account recovery copy', () => {
    const wrapper = mount(ForgotPasswordForm);

    expect(wrapper.text()).toContain('Account Recovery');
    expect(wrapper.text()).toContain('Reset your password');
  });

  it('emits email submit payload', async () => {
    const wrapper = mount(ForgotPasswordForm);

    await wrapper.find('input[type="email"]').setValue('peter@example.com');
    await wrapper.find('form').trigger('submit.prevent');

    expect(wrapper.emitted('submit')).toBeTruthy();
    expect(wrapper.emitted('submit')?.[0]).toEqual([
      {
        email: 'peter@example.com',
      },
    ]);
  });

  it('renders success message when provided', () => {
    const wrapper = mount(ForgotPasswordForm, {
      props: {
        message: 'Reset instructions generated successfully.',
      },
    });

    expect(wrapper.text()).toContain(
      'Reset instructions generated successfully.',
    );
  });

  it('renders error message when provided', () => {
    const wrapper = mount(ForgotPasswordForm, {
      props: {
        error: 'Unable to generate reset link.',
      },
    });

    expect(wrapper.text()).toContain('Unable to generate reset link.');
  });
});

describe('AppHeader', () => {
  it('renders brand and portal links', () => {
    const wrapper = mount(AppHeader, {
      global: {
        stubs: {
          NuxtLink: {
            props: ['to'],
            template: '<a :href="to"><slot /></a>',
          },
        },
      },
    });

    expect(wrapper.text()).toContain('Consult Ops');
    expect(wrapper.text()).toContain('Operational certainty for service work');
    expect(wrapper.text()).toContain('Consultant Login');
    expect(wrapper.text()).toContain('Client Login');
    expect(wrapper.text()).toContain('Admin');
  });

  it('links to the expected login routes', () => {
    const wrapper = mount(AppHeader, {
      global: {
        stubs: {
          NuxtLink: {
            props: ['to'],
            template: '<a :href="to"><slot /></a>',
          },
        },
      },
    });

    expect(wrapper.html()).toContain('href="/consultant-login"');
    expect(wrapper.html()).toContain('href="/client-login"');
    expect(wrapper.html()).toContain('href="/admin-login"');
  });
});

describe('ResetPasswordForm', () => {
  it('renders password reset copy', () => {
    const wrapper = mount(ResetPasswordForm);

    expect(wrapper.text()).toContain('Password Reset');
    expect(wrapper.text()).toContain('Create a new password');
  });

  it('emits submit payload when passwords match', async () => {
    const wrapper = mount(ResetPasswordForm);

    await wrapper.find('#password').setValue('NewStrongPass456!');
    await wrapper.find('#confirmPassword').setValue('NewStrongPass456!');
    await wrapper.find('form').trigger('submit.prevent');

    expect(wrapper.emitted('submit')).toBeTruthy();
    expect(wrapper.emitted('submit')?.[0]).toEqual([
      {
        password: 'NewStrongPass456!',
      },
    ]);
  });

  it('shows error when passwords do not match', async () => {
    const wrapper = mount(ResetPasswordForm);

    await wrapper.find('#password').setValue('NewStrongPass456!');
    await wrapper.find('#confirmPassword').setValue('DifferentPass456!');
    await wrapper.find('form').trigger('submit.prevent');

    expect(wrapper.text()).toContain('Passwords do not match');
    expect(wrapper.emitted('submit')).toBeFalsy();
  });

  it('renders success message when provided', () => {
    const wrapper = mount(ResetPasswordForm, {
      props: {
        message: 'Password reset successful.',
      },
    });

    expect(wrapper.text()).toContain('Password reset successful.');
  });

  it('renders error message when provided', () => {
    const wrapper = mount(ResetPasswordForm, {
      props: {
        error: 'Invalid or expired reset token.',
      },
    });

    expect(wrapper.text()).toContain('Invalid or expired reset token.');
  });
});
describe('RegisterForm', () => {
  it('renders consultant registration copy by default', () => {
    const wrapper = mount(RegisterForm);

    expect(wrapper.text()).toContain('Create your consultant workspace');
    expect(wrapper.text()).toContain(
      'Start managing clients, projects, contracts, invoices, and payments.',
    );
  });

  it('renders client registration copy', () => {
    const wrapper = mount(RegisterForm, {
      props: {
        userType: 'client',
        submitText: 'Create client account',
      },
    });

    expect(wrapper.text()).toContain('Create a client portal account');
    expect(wrapper.text()).toContain('Create client account');
  });

  it('emits registration payload when form is valid', async () => {
    const wrapper = mount(RegisterForm, {
      props: {
        userType: 'consultant',
      },
    });

    await wrapper.find('#name').setValue('Peter');
    await wrapper.find('#email').setValue('peter@example.com');
    await wrapper.find('#password').setValue('StrongPass123!');
    await wrapper.find('#confirmPassword').setValue('StrongPass123!');
    await wrapper.find('form').trigger('submit.prevent');

    expect(wrapper.emitted('submit')).toBeTruthy();
    expect(wrapper.emitted('submit')?.[0]).toEqual([
      {
        name: 'Peter',
        email: 'peter@example.com',
        password: 'StrongPass123!',
        user_type: 'consultant',
      },
    ]);
  });

  it('shows error when passwords do not match', async () => {
    const wrapper = mount(RegisterForm);

    await wrapper.find('#name').setValue('Peter');
    await wrapper.find('#email').setValue('peter@example.com');
    await wrapper.find('#password').setValue('StrongPass123!');
    await wrapper.find('#confirmPassword').setValue('DifferentPass123!');
    await wrapper.find('form').trigger('submit.prevent');

    expect(wrapper.text()).toContain('Passwords do not match');
    expect(wrapper.emitted('submit')).toBeFalsy();
  });

  it('renders server error when provided', () => {
    const wrapper = mount(RegisterForm, {
      props: {
        error: 'Email is already registered.',
      },
    });

    expect(wrapper.text()).toContain('Email is already registered.');
  });
});
