import type { Meta, StoryFn } from '@storybook-vue/nuxt';
import LoginForm from './LoginForm.vue';

const meta: Meta<typeof LoginForm> = {
  title: 'Auth/LoginForm',
  component: LoginForm,
  argTypes: {
    userType: {
      control: 'select',
      options: ['admin', 'client', 'consultant'],
    },
    submitText: {
      control: 'text',
    },
  },
};

export default meta;

const Template: StoryFn<typeof LoginForm> = (args) => ({
  components: { LoginForm },
  setup() {
    const handleSubmit = (payload: unknown) => {
      console.log('Login submitted:', payload);
    };

    return { args, handleSubmit };
  },
  template: `
    <div style="min-height: 100vh; padding: 3rem; background: #050b14;">
      <LoginForm v-bind="args" @submit="handleSubmit" />
    </div>
  `,
});

export const Consultant = Template.bind({});
Consultant.args = {
  userType: 'consultant',
  submitText: 'Enter Workspace',
};

export const Client = Template.bind({});
Client.args = {
  userType: 'client',
  submitText: 'Open Client Portal',
};

export const Admin = Template.bind({});
Admin.args = {
  userType: 'admin',
  submitText: 'Enter Admin Console',
};

export const Prefilled = Template.bind({});
Prefilled.args = {
  userType: 'consultant',
  submitText: 'Login',
  initialEmail: 'peter@example.com',
  initialPassword: 'StrongPass123!',
};
