<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  status: string;
  platformFeeStatus?: string;
}>();

const steps = computed(() => [
  {
    key: 'draft',
    label: 'Engagement Created',
    complete: true,
  },
  {
    key: 'platform_fee_paid',
    label: 'Activated',
    complete:
      props.platformFeeStatus === 'paid' ||
      props.platformFeeStatus === 'waived',
  },
  {
    key: 'contract_sent',
    label: 'Contract Sent',
    complete: [
      'contract_sent',
      'contract_signed',
      'work_in_progress',
      'awaiting_review',
      'paid',
      'completed',
    ].includes(props.status),
  },
  {
    key: 'contract_signed',
    label: 'Signed',
    complete: [
      'contract_signed',
      'work_in_progress',
      'awaiting_review',
      'paid',
      'completed',
    ].includes(props.status),
  },
  {
    key: 'awaiting_review',
    label: 'Work Review',
    complete: ['awaiting_review', 'paid', 'completed'].includes(props.status),
  },
  {
    key: 'paid',
    label: 'Paid',
    complete: ['paid', 'completed'].includes(props.status),
  },
]);
</script>

<template>
  <div class="grid gap-3 md:grid-cols-6">
    <div
      v-for="step in steps"
      :key="step.key"
      class="rounded-xl border p-4"
      :class="step.complete ? 'opacity-100' : 'opacity-40'"
    >
      <div class="text-xl">
        {{ step.complete ? '✓' : '○' }}
      </div>
      <div class="text-sm font-semibold">
        {{ step.label }}
      </div>
    </div>
  </div>
</template>
