import type { Meta, StoryObj } from '@storybook/vue3';
import MilestoneForm from './MilestoneForm.vue';

const meta: Meta<typeof MilestoneForm> = {
  title: 'Engagements/MilestoneForm',
  component: MilestoneForm,
  tags: ['autodocs'],
  args: {
    engagementId: 1,
    mockSubmit: true,
  },
};

export default meta;

type Story = StoryObj<typeof MilestoneForm>;

export const Empty: Story = {};

export const InEngagementContext: Story = {
  render: (args) => ({
    components: { MilestoneForm },
    setup() {
      function handleCreated(milestone: any) {
        console.log('Created milestone:', milestone);
      }

      return { args, handleCreated };
    },
    template: `
      <div style="max-width: 720px; padding: 24px;">
        <MilestoneForm v-bind="args" @created="handleCreated" />
      </div>
    `,
  }),
};
