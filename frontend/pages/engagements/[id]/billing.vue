<!-- frontend/pages/engagements/[id]/billing.vue -->
<script setup lang="ts">
const route = useRoute();
const engagementId = Number(route.params.id);

const loading = ref(false);
const message = ref('');

async function startActivationCheckout() {
  loading.value = true;
  message.value = '';

  try {
    // TODO: wire once backend Stripe route exists
    message.value =
      'Stripe activation checkout endpoint is not wired yet. Add /api/engagements/:id/activation-checkout next.';
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <DashboardShell
    title="Engagement Billing"
    subtitle="Activate the engagement and track contractor payment status."
  >
    <section class="portal-section">
      <p class="eyebrow">Platform Fee</p>
      <h2>$10 engagement activation</h2>
      <p>
        This will eventually open Stripe Checkout and unlock contract sending,
        milestone approval, and payment tracking.
      </p>

      <button
        class="form-button"
        :disabled="loading"
        @click="startActivationCheckout"
      >
        {{ loading ? 'Preparing checkout...' : 'Activate Engagement — $10' }}
      </button>

      <p v-if="message">{{ message }}</p>
    </section>
  </DashboardShell>
</template>
