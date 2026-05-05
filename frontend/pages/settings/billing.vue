<script setup lang="ts">
import { ref } from 'vue';

const loading = ref(false);
const plan = ref('free');
const usage = ref({
  engagements_this_month: 2,
  limit: 10,
});

async function startSubscription() {
  loading.value = true;

  try {
    // later: call backend stripe endpoint
    const res = await $fetch(
      'http://127.0.0.1:8000/api/billing/create-checkout',
    );

    // redirect to Stripe
    window.location.href = res.url;
  } catch (err) {
    console.error(err);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <main class="p-6 max-w-4xl mx-auto">
    <div class="form-shell">
      <div>
        <p class="form-eyebrow">Billing</p>
        <h1 class="form-title">Subscription & Usage</h1>
        <p class="form-subtitle">
          Manage your plan, payments, and platform usage.
        </p>
      </div>

      <!-- PLAN -->
      <section class="space-y-3">
        <h2 class="text-white text-lg font-bold">Current Plan</h2>

        <div class="rounded-xl border border-slate-600 p-4 bg-slate-900">
          <p class="text-slate-300">
            Plan:
            <span class="text-white font-bold capitalize">{{ plan }}</span>
          </p>

          <p class="text-slate-400 text-sm mt-1">
            $10 per engagement or upgrade to unlimited
          </p>
        </div>
      </section>

      <!-- USAGE -->
      <section class="space-y-3">
        <h2 class="text-white text-lg font-bold">Usage</h2>

        <div class="rounded-xl border border-slate-600 p-4 bg-slate-900">
          <p class="text-slate-300">
            {{ usage.engagements_this_month }} / {{ usage.limit }} engagements
            this month
          </p>

          <div class="w-full bg-slate-800 rounded-full h-2 mt-2">
            <div
              class="bg-gradient-to-r from-sky-400 to-emerald-400 h-2 rounded-full"
              :style="{
                width: (usage.engagements_this_month / usage.limit) * 100 + '%',
              }"
            />
          </div>
        </div>
      </section>

      <!-- ACTION -->
      <section class="space-y-3">
        <h2 class="text-white text-lg font-bold">Upgrade</h2>

        <button
          class="form-button"
          @click="startSubscription"
          :disabled="loading"
        >
          {{ loading ? 'Redirecting...' : 'Upgrade to Pro ($15/mo)' }}
        </button>
      </section>
    </div>
  </main>
</template>
