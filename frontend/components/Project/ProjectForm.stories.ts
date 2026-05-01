import type { Meta, StoryObj } from '@storybook/vue3-vite';
import ProjectForm from './ProjectForm.vue';

const meta: Meta<typeof ProjectForm> = {
  title: 'ERP/Projects/ProjectForm',
  component: ProjectForm,
  parameters: {
    layout: 'centered',
  },
};

export default meta;
type Story = StoryObj<typeof ProjectForm>;

export const Default: Story = {};

export const PlanningStage: Story = {
  render: () => ({
    components: { ProjectForm },
    template: '<ProjectForm />',
  }),
};
