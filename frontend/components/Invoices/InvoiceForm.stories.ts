import type { Meta, StoryObj } from '@storybook/vue3';
import InvoiceForm from './InvoiceForm.vue';

const meta: Meta<typeof InvoiceForm> = {
  title: 'ERP/Invoices/InvoiceForm',
  component: InvoiceForm,
  parameters: {
    layout: 'centered',
  },
};

export default meta;
type Story = StoryObj<typeof InvoiceForm>;

export const Default: Story = {};

export const Prefilled: Story = {
  render: () => ({
    components: { InvoiceForm },
    template: '<InvoiceForm />',
    mounted() {
      console.log('Simulate prefilled invoice here later');
    },
  }),
};
