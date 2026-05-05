<script setup lang="ts">
import { useRoute, useRouter } from 'nuxt/app';
import { computed } from 'vue';

const route = useRoute();
const router = useRouter();

const sessionId = computed(() => String(route.query.session_id || ''));
const engagementId = computed(() => String(route.query.engagement_id || ''));

function goToEngagement() {
  if (engagementId.value) {
    router.push(`/engagements/${engagementId.value}`);
    return;
  }

  router.push('/settings/billing');
}
</script>

<template>
  <main class="p-6 max-w-3xl mx-auto">
    <section class="form-shell">
      <div>
        <p class="form-eyebrow">Payment Complete</p>
        <h1 class="form-title">Billing confirmed</h1>
        <p class="form-subtitle">
          Your payment was successful. The engagement can now move forward.
        </p>
      </div>

      <div
        class="rounded-xl border border-emerald-400/40 bg-emerald-950/30 p-4 text-emerald-100"
      >
        <p class="font-bold">Activation successful</p>
        <p v-if="sessionId" class="mt-1 text-sm opacity-80">
          Stripe session: {{ sessionId }}
        </p>
      </div>

      <button class="form-button" @click="goToEngagement">
        {{ engagementId ? 'Return to Engagement' : 'Go to Billing Settings' }}
      </button>
    </section>
  </main>
</template>
