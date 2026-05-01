import type { Meta, StoryFn } from '@storybook-vue/nuxt';
import AppHeader from './AppHeader.vue';

const meta: Meta<typeof AppHeader> = {
  title: 'Layout/AppHeader',
  component: AppHeader,
};

export default meta;

const Template: StoryFn<typeof AppHeader> = () => ({
  components: { AppHeader },
  template: `
    <div style="min-height: 100vh; background: #050b14;">
      <AppHeader />
    </div>
  `,
});

export const Default = Template.bind({});
