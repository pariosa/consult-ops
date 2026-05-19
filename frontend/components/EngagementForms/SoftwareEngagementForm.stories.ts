import type { Meta, StoryObj } from '@storybook/vue3';
import SoftwareEngagementForm from './EngagementForms/SoftwareEngagementForm.vue';

const meta: Meta<typeof SoftwareEngagementForm> = {
  title: 'Engagements/SoftwareEngagementForm',
  component: SoftwareEngagementForm,
  tags: ['autodocs'],
  args: {
    projectId: 1,
    organizationId: 1,
    mockSubmit: true,
  },
};

export default meta;

type Story = StoryObj<typeof SoftwareEngagementForm>;

export const Empty: Story = {};

export const ClientPortalExample: Story = {
  render: (args) => ({
    components: { SoftwareEngagementForm },
    setup() {
      function handleCreated(engagement: any) {
        console.log('Created engagement:', engagement);
      }

      return { args, handleCreated };
    },
    template: `
  <div style="min-height: 100vh; background: #020617; padding: 32px;">
    <div style="max-width: 760px;">
      <SoftwareEngagementForm v-bind="args" @created="handleCreated" />
    </div>
  </div>
`,
  }),
};
