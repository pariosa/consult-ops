import type { Meta, StoryObj } from '@storybook/vue3';
import PaymentForm from './PaymentForm.vue';

const meta: Meta<typeof PaymentForm> = {
  title: 'ERP/Payments/PaymentForm',
  component: PaymentForm,
  parameters: {
    layout: 'centered',
  },
};

export default meta;
type Story = StoryObj<typeof PaymentForm>;

export const Default: Story = {};

export const WithInvoiceContext: Story = {
  render: () => ({
    components: { PaymentForm },
    template: '<PaymentForm />',
  }),
};
