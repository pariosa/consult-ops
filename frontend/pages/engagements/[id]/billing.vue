<script setup lang="ts">
import { ref } from 'vue';
import { useEngagements } from '~/composables/useEngagements';
import { useEngagementMilestones } from '~/composables/useEngagementMilestones';

const route = useRoute();
const engagementId = Number(route.params.id);

const { getEngagement } = useEngagements();
const { getMilestones } = useEngagementMilestones();

const engagement = ref<any>(null);
const milestones = ref<any[]>([]);
const loading = ref(true);
const checkoutLoading = ref(false);
const message = ref('');

async function refresh() {
  loading.value = true;

  try {
    engagement.value = await getEngagement(engagementId);
    milestones.value = await getMilestones(engagementId);
  } finally {
    loading.value = false;
  }
}

const totalMilestoneCents = computed(() =>
  milestones.value.reduce(
    (sum, item) => sum + Number(item.amount_cents || 0),
    0,
  ),
);

const paidMilestoneCents = computed(() =>
  milestones.value
    .filter((item) => item.status === 'paid')
    .reduce((sum, item) => sum + Number(item.amount_cents || 0), 0),
);

const remainingMilestoneCents = computed(
  () => totalMilestoneCents.value - paidMilestoneCents.value,
);

function formatCurrency(cents: number) {
  return `$${(cents / 100).toFixed(2)}`;
}

async function startActivationCheckout() {
  checkoutLoading.value = true;
  message.value = '';

  try {
    message.value =
      'Stripe activation checkout endpoint is not wired yet. Add the backend checkout route next.';
  } finally {
    checkoutLoading.value = false;
  }
}

await refresh();
</script>

<template>
  <DashboardShell
    title="Engagement Billing"
    subtitle="Activate the engagement and track contractor payment status."
  >
    <section v-if="loading" class="portal-section">
      Loading billing workflow...
    </section>

    <template v-else>
      <section class="portal-section">
        <NuxtLink
          :to="`/engagements/${engagementId}`"
          class="text-cyan-300 hover:text-cyan-200"
        >
          ← Back to engagement
        </NuxtLink>

        <div class="section-header mt-4">
          <div>
            <p class="eyebrow">Billing Overview</p>
            <h2 class="text-2xl font-bold text-white">
              {{ engagement?.title || 'Engagement' }}
            </h2>
            <p class="text-slate-300">
              {{ engagement?.contractor_name }} —
              {{ engagement?.contractor_email }}
            </p>
          </div>

          <span class="status-pill">
            {{ engagement?.platform_fee_status || 'pending' }}
          </span>
        </div>
      </section>

      <section class="portal-section">
        <div class="section-header">
          <div>
            <p class="eyebrow">Platform Fee</p>
            <h2 class="text-2xl font-bold text-white">
              $10 engagement activation
            </h2>
            <p class="text-slate-300">
              Activation will unlock contract sending, milestone approval, and
              payment tracking.
            </p>
          </div>
        </div>

        <button
          class="form-button"
          type="button"
          :disabled="checkoutLoading"
          @click="startActivationCheckout"
        >
          {{
            checkoutLoading
              ? 'Preparing checkout...'
              : 'Activate Engagement — $10'
          }}
        </button>

        <p v-if="message" class="text-slate-300 mt-3">
          {{ message }}
        </p>
      </section>

      <section class="portal-section">
        <div class="section-header">
          <div>
            <p class="eyebrow">Milestone Payments</p>
            <h2 class="text-2xl font-bold text-white">Payment Summary</h2>
          </div>
        </div>

        <div class="billing-grid">
          <div class="ops-card">
            <p class="eyebrow">Total Milestones</p>
            <h3 class="text-xl font-bold text-white">
              {{ formatCurrency(totalMilestoneCents) }}
            </h3>
          </div>

          <div class="ops-card">
            <p class="eyebrow">Paid</p>
            <h3 class="text-xl font-bold text-white">
              {{ formatCurrency(paidMilestoneCents) }}
            </h3>
          </div>

          <div class="ops-card">
            <p class="eyebrow">Remaining</p>
            <h3 class="text-xl font-bold text-white">
              {{ formatCurrency(remainingMilestoneCents) }}
            </h3>
          </div>
        </div>

        <div v-if="!milestones.length" class="empty-state">
          No milestones available for billing yet.
        </div>

        <div
          v-for="milestone in milestones"
          :key="milestone.id"
          class="ops-card milestone-card"
        >
          <div class="flex items-center justify-between mb-4">
            <div>
              <h3 class="text-xl font-semibold text-white">
                {{ milestone.title }}
              </h3>

              <p class="text-slate-300 mt-1">
                {{ milestone.description || 'No description provided.' }}
              </p>
            </div>

            <div class="text-right">
              <p class="text-cyan-200 font-bold text-lg">
                {{ formatCurrency(milestone.amount_cents) }}
              </p>

              <span class="status-pill mt-2 inline-flex">
                {{ milestone.status }}
              </span>
            </div>
          </div>
        </div>
      </section>
    </template>
  </DashboardShell>
</template>
