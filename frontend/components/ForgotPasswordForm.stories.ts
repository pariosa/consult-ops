import type { Meta, StoryFn } from '@storybook-vue/nuxt';
import ForgotPasswordForm from './ForgotPasswordForm.vue';

const meta: Meta<typeof ForgotPasswordForm> = {
  title: 'Auth/ForgotPasswordForm',
  component: ForgotPasswordForm,
};

export default meta;

const Template: StoryFn<typeof ForgotPasswordForm> = (args) => ({
  components: { ForgotPasswordForm },
  setup() {
    const handleSubmit = (payload: unknown) => {
      console.log('Forgot password submitted:', payload);
    };

    return { args, handleSubmit };
  },
  template: `
    <div style="min-height: 100vh; padding: 3rem; background: #050b14;">
      <ForgotPasswordForm v-bind="args" @submit="handleSubmit" />
    </div>
  `,
});

export const Default = Template.bind({});
Default.args = {};

export const Success = Template.bind({});
Success.args = {
  message: 'Reset instructions generated successfully.',
};

export const Error = Template.bind({});
Error.args = {
  error: 'Unable to generate reset link.',
};
