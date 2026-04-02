import LoginForm from './LoginForm.vue';
import type { Meta, StoryFn } from '@storybook-vue/nuxt';

const meta: Meta<typeof LoginForm> = {
  title: 'Components/LoginForm',
  component: LoginForm,
};
export default meta;

const Template: StoryFn<typeof LoginForm> = (args) => ({
  components: { LoginForm },
  setup() { return { args }; },
  template: '<LoginForm v-bind="args" />',
});

export const Admin = Template.bind({});
Admin.args = {
  userType: 'admin',
};

export const Client = Template.bind({});
Client.args = {
  userType: 'client',
};

export const Consultant = Template.bind({});
Consultant.args = {
  userType: 'consultant',
};