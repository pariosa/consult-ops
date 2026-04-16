import type { Meta, StoryObj } from '@storybook/vue3';
import ContractForm from './ContractForm.vue';

const meta: Meta<typeof ContractForm> = {
  title: 'ERP/Contracts/ContractForm',
  component: ContractForm,
  parameters: {
    layout: 'centered',
  },
};

export default meta;
type Story = StoryObj<typeof ContractForm>;

export const Default: Story = {};

export const DraftContract: Story = {
  render: () => ({
    components: { ContractForm },
    template: '<ContractForm />',
  }),
};
