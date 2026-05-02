import type { Meta, StoryFn } from '@storybook-vue/nuxt';
import RegisterForm from './RegisterForm.vue';

const meta: Meta<typeof RegisterForm> = {
  title: 'Auth/RegisterForm',
  component: RegisterForm,
  argTypes: {
    userType: {
      control: 'select',
      options: ['consultant', 'client', 'admin'],
    },
  },
};

export default meta;

const Template: StoryFn<typeof RegisterForm> = (args) => ({
  components: { RegisterForm },
  setup() {
    const handleSubmit = (payload: unknown) => {
      console.log('Register submitted:', payload);
    };

    return { args, handleSubmit };
  },
  template: `
    <div style="min-height: 100vh; padding: 3rem; background: #050b14;">
      <RegisterForm v-bind="args" @submit="handleSubmit" />
    </div>
  `,
});

export const Consultant = Template.bind({});
Consultant.args = {
  userType: 'consultant',
  submitText: 'Create consultant workspace',
};

export const Client = Template.bind({});
Client.args = {
  userType: 'client',
  submitText: 'Create client account',
};

export const Admin = Template.bind({});
Admin.args = {
  userType: 'admin',
  submitText: 'Create admin account',
};

export const Success = Template.bind({});
Success.args = {
  userType: 'consultant',
  message: 'Account created successfully.',
};

export const Error = Template.bind({});
Error.args = {
  userType: 'consultant',
  error: 'Email is already registered.',
};
