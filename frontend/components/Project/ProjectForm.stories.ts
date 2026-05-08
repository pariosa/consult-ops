import type { Meta, StoryObj } from '@storybook/vue3-vite';
import ProjectForm from './ProjectForm.vue';

const meta: Meta<typeof ProjectForm> = {
  title: 'ERP/Projects/ProjectForm',
  component: ProjectForm,
  parameters: {
    layout: 'centered',
  },
  args: {
    clients: [
      { id: 1, name: 'Atlas Client', company_name: 'Atlas Studio' },
      { id: 2, name: 'Ribbert Industries', company_name: 'Ribbert Labs' },
    ],
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
