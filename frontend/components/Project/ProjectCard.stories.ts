import type { Meta, StoryObj } from '@storybook/vue3';
import ProjectCard from './ProjectCard.vue';

const meta: Meta<typeof ProjectCard> = {
  title: 'ERP/Projects/ProjectCard',
  component: ProjectCard,
};

export default meta;
type Story = StoryObj<typeof ProjectCard>;

const base = {
  id: 1,
  client_id: 200,
  name: 'ERP System Build',
  start_date: '2026-03-01',
  end_date: null,
  description: 'Full stack ERP development',
  created_at: '2026-02-15',
};

export const Active: Story = {
  args: { project: { ...base } },
};

export const Draft: Story = {
  args: {
    project: {
      ...base,
      start_date: null,
    },
  },
};

export const Completed: Story = {
  args: {
    project: {
      ...base,
      end_date: '2026-05-01',
    },
  },
};
