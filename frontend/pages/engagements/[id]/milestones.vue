<!-- frontend/pages/engagements/[id]/milestones.vue -->
<script setup lang="ts">
import { ref } from 'vue';
import MilestoneForm from '~/components/Engagements/MilestoneForm.vue';
import { useEngagementMilestones } from '~/composables/useEngagementMilestones';
import { useEngagements } from '~/composables/useEngagements';

const route = useRoute();
const engagementId = Number(route.params.id);

const showCreateForm = ref(false);
const editingMilestoneId = ref<number | null>(null);

const editForm = ref({
  title: '',
  description: '',
  amount_cents: 0,
});

const { getEngagement } = useEngagements();

const {
  getMilestones,
  submitMilestone,
  approveMilestone,
  markMilestonePaid,
  createMilestone,
  updateMilestone,
  reopenMilestone,
} = useEngagementMilestones();

const engagement = ref<any>(null);
const milestones = ref<any[]>([]);

async function refresh() {
  engagement.value = await getEngagement(engagementId);
  milestones.value = await getMilestones(engagementId);
}

function startEditing(milestone: any) {
  editingMilestoneId.value = milestone.id;
  editForm.value = {
    title: milestone.title,
    description: milestone.description ?? '',
    amount_cents: milestone.amount_cents,
  };
}

function cancelEditing() {
  editingMilestoneId.value = null;
}

async function saveMilestone(id: number) {
  await updateMilestone(id, editForm.value);
  editingMilestoneId.value = null;
  await refresh();
}

async function reopen(id: number) {
  await reopenMilestone(id);
  await refresh();
}

async function createMilestoneLocal(payload: any) {
  await createMilestone(engagementId, payload);
  showCreateForm.value = false;
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
  <DashboardShell
    title="Milestone Management"
    subtitle="Track deliverables, approvals, and contractor payouts."
  >
    <section class="portal-section">
      <NuxtLink
        :to="`/engagements/${engagementId}`"
        class="text-cyan-300 hover:text-cyan-200"
      >
        ← Back to engagement
      </NuxtLink>

      <div class="section-header mt-4">
        <div>
          <p class="eyebrow">Engagement</p>
          <h2 class="text-2xl font-bold text-white">
            {{ engagement?.title || 'Engagement' }}
          </h2>
        </div>

        <button
          class="form-button"
          type="button"
          @click="showCreateForm = !showCreateForm"
        >
          {{ showCreateForm ? 'Close Form' : 'Add Milestone' }}
        </button>
      </div>
    </section>

    <section v-if="showCreateForm" class="portal-section">
      <MilestoneForm
        :engagement-id="engagementId"
        @submit="createMilestoneLocal"
      />
    </section>

    <section class="portal-section">
      <div class="section-header">
        <div>
          <h2 class="text-2xl font-bold text-white">Milestones</h2>
          <p class="text-slate-300">
            Submit work, approve milestones, and mark payouts complete.
          </p>
        </div>
      </div>

      <div v-if="!milestones.length" class="empty-state">
        No milestones created yet.
      </div>

      <div
        v-for="milestone in milestones"
        :key="milestone.id"
        class="ops-card milestone-card"
      >
        <div v-if="editingMilestoneId === milestone.id" class="space-y-3">
          <input
            v-model="editForm.title"
            class="form-input"
            placeholder="Milestone title"
          />

          <textarea
            v-model="editForm.description"
            class="form-input"
            placeholder="Milestone description"
          />

          <input
            v-model.number="editForm.amount_cents"
            type="number"
            class="form-input"
            placeholder="Amount in cents"
          />

          <div class="milestone-actions">
            <button
              class="form-button"
              type="button"
              @click="saveMilestone(milestone.id)"
            >
              Save Changes
            </button>

            <button
              class="form-button secondary-button"
              type="button"
              @click="cancelEditing"
            >
              Cancel
            </button>
          </div>
        </div>

        <div v-else class="space-y-3">
          <div class="flex items-center justify-between gap-4">
            <h3 class="text-xl font-semibold text-white">
              {{ milestone.title }}
            </h3>

            <span class="status-pill">
              {{ milestone.status }}
            </span>
          </div>

          <p class="text-slate-300">
            {{ milestone.description || 'No description provided.' }}
          </p>

          <p class="text-cyan-200 font-semibold">
            ${{ (milestone.amount_cents / 100).toFixed(2) }}
          </p>

          <div class="milestone-actions">
            <button
              v-if="milestone.status !== 'paid'"
              class="form-button"
              type="button"
              @click="startEditing(milestone)"
            >
              Edit
            </button>

            <button
              v-if="milestone.status === 'pending'"
              data-testid="submit-milestone-button"
              class="form-button"
              type="button"
              @click="submit(milestone.id)"
            >
              Submit
            </button>

            <button
              v-if="milestone.status === 'submitted'"
              data-testid="approve-milestone-button"
              class="form-button"
              type="button"
              @click="approve(milestone.id)"
            >
              Approve
            </button>

            <button
              v-if="milestone.status === 'approved'"
              class="form-button"
              type="button"
              @click="markPaid(milestone.id)"
            >
              Mark Paid
            </button>

            <button
              v-if="
                milestone.status === 'approved' || milestone.status === 'paid'
              "
              class="form-button secondary-button"
              type="button"
              @click="reopen(milestone.id)"
            >
              Reopen
            </button>
          </div>
        </div>
      </div>
    </section>
  </DashboardShell>
</template>
