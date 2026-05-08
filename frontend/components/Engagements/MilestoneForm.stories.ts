import type { Meta, StoryObj } from '@storybook/vue3';
import MilestoneForm from './MilestoneForm.vue';

const meta: Meta<typeof MilestoneForm> = {
  title: 'Engagements/MilestoneForm',
  component: MilestoneForm,
  tags: ['autodocs'],
  args: {
    engagementId: 1,
    loading: false,
  },
  argTypes: {
    submit: {
      action: 'submit',
    },
    created: {
      action: 'created',
    },
  },
};

export default meta;

type Story = StoryObj<typeof MilestoneForm>;

export const Default: Story = {
  args: {
    engagementId: 1,
  },
};

export const Loading: Story = {
  args: {
    engagementId: 1,
    loading: true,
  },
};

export const InEngagementContext: Story = {
  args: {
    engagementId: 42,
  },

  render: (args) => ({
    components: { MilestoneForm },

    setup() {
      return { args };
    },

    template: `
      <div
        style="
          min-height: 100vh;
          padding: 48px;
          background: #020617;
        "
      >
        <div
          style="
            max-width: 720px;
            margin: 0 auto;
          "
        >
          <MilestoneForm
            v-bind="args"
            @submit="args.submit"
            @created="args.created"
          />
        </div>
      </div>
    `,
  }),
};
