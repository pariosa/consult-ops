<script setup lang="ts">
import { useRoute } from 'nuxt/app';
import type { ref } from 'process';
import { onMounted } from 'vue';
import { useEngagementMilestones } from '~/composables/useEngagementMilestones';
import { useEngagements } from '~/composables/useEngagements';
import EngagementTracker from '~/components/Engagements/EngagementTracker.vue';
import SoftwareContractPreview from '~/components/Contracts/SoftwareContractPreview.vue';

const route = useRoute();
const engagementId = Number(route.params.id);

const {
  getEngagement,
  generateSoftwareContract,
  markContractSent,
  markSigned,
} = useEngagements();

const { getMilestones, submitMilestone, approveMilestone, markMilestonePaid } =
  useEngagementMilestones();

const engagement = ref<any>(null);
const milestones = ref<any[]>([]);
const contractPreview = ref('');
const loading = ref(true);
const error = ref('');

async function refresh() {
  loading.value = true;
  error.value = '';

  try {
    engagement.value = await getEngagement(engagementId);
    milestones.value = await getMilestones(engagementId);
  } catch (err: any) {
    error.value = err?.message || 'Failed to load engagement.';
  } finally {
    loading.value = false;
  }
}

async function previewContract() {
  const res: any = await generateSoftwareContract(engagementId);
  contractPreview.value = res.body;
}

async function sendContract() {
  engagement.value = await markContractSent(engagementId);
}

async function markSignedLocal() {
  engagement.value = await markSigned(engagementId);
}

async function submitMilestoneLocal(id: number) {
  await submitMilestone(id);
  await refresh();
}

async function approveMilestoneLocal(id: number) {
  await approveMilestone(id);
  await refresh();
}

async function markPaidLocal(id: number) {
  await markMilestonePaid(id);
  await refresh();
}

onMounted(refresh);
</script>

<template>
  <DashboardShell
    title="Engagement Tracker"
    subtitle="Progress contract, milestone, and payment workflow."
  >
    <section v-if="loading" class="portal-section">
      Loading engagement...
    </section>

    <section v-else-if="error" class="form-error">
      {{ error }}
    </section>

    <template v-else>
      <section class="portal-section">
        <p class="eyebrow">Software Engagement</p>
        <h2>{{ engagement.title }}</h2>
        <p>
          {{ engagement.contractor_name }} — {{ engagement.contractor_email }}
        </p>
        <p>{{ engagement.scope_of_work }}</p>
      </section>

      <EngagementTracker
        :status="engagement.status"
        :platform-fee-status="engagement.platform_fee_status"
      />

      <section class="portal-section action-grid">
        <button class="form-button" @click="previewContract">
          Generate Contract Preview
        </button>

        <button class="form-button" @click="sendContract">
          Mark Contract Sent
        </button>

        <button class="form-button" @click="markSignedLocal">
          Mark Contract Signed
        </button>

        <NuxtLink
          class="form-button link-button"
          :to="`/engagements/${engagementId}/milestones`"
        >
          Manage Milestones
        </NuxtLink>

        <NuxtLink
          class="form-button link-button"
          :to="`/engagements/${engagementId}/billing`"
        >
          Payment Workflow
        </NuxtLink>
      </section>

      <SoftwareContractPreview v-if="contractPreview" :body="contractPreview" />

      <section class="portal-section">
        <div class="section-header">
          <h2>Milestones</h2>
          <p>Submit, approve, and mark work as paid.</p>
        </div>

        <div v-if="!milestones.length">No milestones yet.</div>

        <div
          v-for="milestone in milestones"
          :key="milestone.id"
          class="ops-card milestone-row"
        >
          <div>
            <h3>{{ milestone.title }}</h3>
            <p>{{ milestone.description }}</p>
            <p>Status: {{ milestone.status }}</p>
          </div>

          <div class="milestone-actions">
            <button @click="submitMilestoneLocal(milestone.id)">Submit</button>
            <button @click="approveMilestoneLocal(milestone.id)">
              Approve
            </button>
            <button @click="markPaidLocal(milestone.id)">Mark Paid</button>
          </div>
        </div>
      </section>
    </template>
  </DashboardShell>
</template>
