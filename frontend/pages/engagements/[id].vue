<script setup lang="ts">
import { useRoute } from 'nuxt/app';
import type { ref } from 'process';
import { useEngagementMilestones } from '~/composables/useEngagementMilestones';
import { useEngagements } from '~/composables/useEngagements';

const route = useRoute();

const engagementId = Number(route.params.id);

const {
  getEngagement,
  generateSoftwareContract,
  markContractSent,
  markSigned,
} = useEngagements();

const { getMilestones } = useEngagementMilestones();

const engagement = ref<any>(null);
const milestones = ref<any[]>([]);
const contractPreview = ref<string>('');

async function refresh() {
  engagement.value = await getEngagement(engagementId);
  milestones.value = await getMilestones(engagementId);
}

async function previewContract() {
  const response = await generateSoftwareContract(engagementId);
  contractPreview.value = response.body;
}

async function sendContract() {
  engagement.value = await markContractSent(engagementId);
}

async function signContract() {
  engagement.value = await markSigned(engagementId);
}

await refresh();
</script>

<template>
  <main v-if="engagement" class="p-6 space-y-6">
    <section>
      <h1 class="text-2xl font-bold">{{ engagement.title }}</h1>
      <p class="opacity-75">
        {{ engagement.contractor_name }} — {{ engagement.role }}
      </p>
    </section>

    <EngagementTracker
      :status="engagement.status"
      :platform-fee-status="engagement.platform_fee_status"
    />

    <section class="rounded-xl border p-4 space-y-3">
      <h2 class="font-bold">Actions</h2>

      <button class="btn" @click="previewContract">
        Preview Software Contract
      </button>

      <button class="btn" @click="sendContract">Mark Contract Sent</button>

      <button class="btn" @click="signContract">Mark Signed</button>
    </section>

    <section v-if="contractPreview" class="rounded-xl border p-4">
      <h2 class="font-bold mb-3">Contract Preview</h2>
      <pre class="whitespace-pre-wrap text-sm">{{ contractPreview }}</pre>
    </section>

    <section class="rounded-xl border p-4">
      <h2 class="font-bold mb-3">Milestones</h2>

      <div v-if="!milestones.length" class="opacity-70">No milestones yet.</div>

      <div
        v-for="milestone in milestones"
        :key="milestone.id"
        class="border rounded-lg p-3 mb-2"
      >
        <div class="font-semibold">{{ milestone.title }}</div>
        <div class="text-sm opacity-75">{{ milestone.description }}</div>
        <div class="text-sm">{{ milestone.status }}</div>
      </div>
    </section>
  </main>
</template>
