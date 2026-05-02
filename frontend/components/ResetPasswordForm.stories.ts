import type { Meta, StoryFn } from '@storybook-vue/nuxt';
import ResetPasswordForm from './ResetPasswordForm.vue';

const meta: Meta<typeof ResetPasswordForm> = {
  title: 'Auth/ResetPasswordForm',
  component: ResetPasswordForm,
};

export default meta;

const Template: StoryFn<typeof ResetPasswordForm> = (args) => ({
  components: { ResetPasswordForm },
  setup() {
    const handleSubmit = (payload: unknown) => {
      console.log('Reset password submitted:', payload);
    };

    return { args, handleSubmit };
  },
  template: `
    <div style="min-height: 100vh; padding: 3rem; background: #050b14;">
      <ResetPasswordForm v-bind="args" @submit="handleSubmit" />
    </div>
  `,
});

export const Default = Template.bind({});
Default.args = {};

export const Success = Template.bind({});
Success.args = {
  message: 'Password reset successful.',
};

export const Error = Template.bind({});
Error.args = {
  error: 'Invalid or expired reset token.',
};
