<script setup lang="ts">
import { useRoute } from 'nuxt/app';
import { ref } from 'process';
import { useEngagementMilestones } from '~/composables/useEngagementMilestones';
import { useEngagements } from '~/composables/useEngagements';

const route = useRoute();
const engagementId = Number(route.params.id);

const { getEngagement } = useEngagements();
const { getMilestones, submitMilestone, approveMilestone, markMilestonePaid } =
  useEngagementMilestones();

const engagement = ref<any>(null);
const milestones = ref<any[]>([]);

async function refresh() {
  engagement.value = await getEngagement(engagementId);
  milestones.value = await getMilestones(engagementId);
}

async function onCreated() {
  await refresh();
}

async function submit(id: number) {
  await submitMilestone(id);
  await refresh();
}

async function approve(id: number) {
  await approveMilestone(id);
  await refresh();
}

async function markPaid(id: number) {
  await markMilestonePaid(id);
  await refresh();
}

await refresh();
</script>

<template>
  <main class="p-6 max-w-5xl mx-auto space-y-6">
    <section v-if="engagement">
      <NuxtLink :to="`/engagements/${engagementId}`" class="text-sm opacity-75">
        ← Back to engagement
      </NuxtLink>

      <h1 class="text-2xl font-bold mt-2">
        Milestones for {{ engagement.title }}
      </h1>
    </section>

    <MilestoneForm :engagement-id="engagementId" @created="onCreated" />

    <section class="rounded-xl border p-4">
      <h2 class="text-lg font-bold mb-4">Milestone Tracker</h2>

      <div v-if="!milestones.length" class="opacity-70">No milestones yet.</div>

      <div
        v-for="milestone in milestones"
        :key="milestone.id"
        class="rounded-lg border p-4 mb-3 space-y-2"
      >
        <div class="flex justify-between gap-4">
          <div>
            <h3 class="font-bold">{{ milestone.title }}</h3>
            <p class="text-sm opacity-75">{{ milestone.description }}</p>
            <p class="text-sm">
              Amount: ${{ (milestone.amount_cents / 100).toFixed(2) }}
            </p>
            <p class="text-sm">Status: {{ milestone.status }}</p>
          </div>

          <div class="flex gap-2 items-start">
            <button class="btn" @click="submit(milestone.id)">Submit</button>
            <button class="btn" @click="approve(milestone.id)">Approve</button>
            <button class="btn" @click="markPaid(milestone.id)">
              Mark Paid
            </button>
          </div>
        </div>
      </div>
    </section>
  </main>
</template>
