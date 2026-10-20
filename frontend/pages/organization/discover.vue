
<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import DashboardShell from '~/components/DashboardShell.vue';
import {
  useOrganizationOnboarding,
  type DiscoverableOrganization,
  type MyJoinRequest,
} from '~/composables/useOrganizationOnboarding';

const onboarding = useOrganizationOnboarding();

const organizations = ref<DiscoverableOrganization[]>([]);
const applications = ref<MyJoinRequest[]>([]);
const memberships = ref<number[]>([]);

const search = ref('');
const loading = ref(true);
const error = ref('');
const success = ref('');
const submittingId = ref<number | null>(null);

const filteredOrganizations = computed(() => {
  const query = search.value.trim().toLowerCase();

  if (!query) return organizations.value;

  return organizations.value.filter((org) =>
    org.name.toLowerCase().includes(query),
  );
});

function applicationFor(organizationId: number) {
  return applications.value.find(
    (application) =>
      application.organization_id === organizationId &&
      application.status === 'pending',
  );
}

function isMember(organizationId: number) {
  return memberships.value.includes(organizationId);
}

async function load() {
  loading.value = true;
  error.value = '';

  try {
    const [orgs, requests, mine] = await Promise.all([
      onboarding.discoverOrganizations(),
      onboarding.getMyJoinRequests(),
      onboarding.getMyOrganizations(),
    ]);

    organizations.value = orgs;
    applications.value = requests;
    memberships.value = mine.map((org) => org.organization_id);
  } catch (err: any) {
    error.value =
      err?.message || 'Unable to load public organizations.';
  } finally {
    loading.value = false;
  }
}

async function join(organizationId: number) {
  submittingId.value = organizationId;
  error.value = '';
  success.value = '';

  try {
    await onboarding.requestToJoin(organizationId);

    applications.value = await onboarding.getMyJoinRequests();

    success.value = 'Your membership request has been submitted.';
  } catch (err: any) {
    error.value =
      err?.message || 'Unable to submit membership request.';
  } finally {
    submittingId.value = null;
  }
}

onMounted(load);
</script>

<template>
  <DashboardShell
    title="Discover Organizations"
    subtitle="Find public workspaces and request membership."
  >
    <section class="discovery-panel">
      <div class="heading">
        <div>
          <p class="eyebrow">Organization directory</p>
          <h2>Find your next workspace</h2>
          <p>
            Browse organizations accepting membership requests.
          </p>
        </div>

        <NuxtLink to="/organization" class="secondary-link">
          Back to HQ
        </NuxtLink>
      </div>

      <label for="organization-search" class="search-label">
        Search organizations
      </label>

      <input
        id="organization-search"
        v-model="search"
        class="search-input"
        type="search"
        placeholder="Search by organization name..."
      />

      <p v-if="success" class="success-message" role="status">
        {{ success }}
      </p>

      <p v-if="error" class="error-message" role="alert">
        {{ error }}
      </p>

      <p v-if="loading">Loading organizations...</p>

      <p
        v-else-if="filteredOrganizations.length === 0"
        class="empty-state"
      >
        No public organizations found.
      </p>

      <div v-else class="organization-grid">
        <article
          v-for="org in filteredOrganizations"
          :key="org.id"
          class="organization-card"
        >
          <div>
            <p class="eyebrow">Public organization</p>
            <h3>{{ org.name }}</h3>
            <p v-if="org.slug" class="slug">{{ org.slug }}</p>
          </div>

          <span v-if="isMember(org.id)" class="status member">
            Member
          </span>

          <span
            v-else-if="applicationFor(org.id)"
            class="status pending"
          >
            Request pending
          </span>

          <button
            v-else
            type="button"
            class="join-button"
            :disabled="submittingId !== null"
            @click="join(org.id)"
          >
            {{
              submittingId === org.id
                ? 'Submitting...'
                : 'Request to join'
            }}
          </button>
        </article>
      </div>
    </section>
  </DashboardShell>
</template>

<style scoped>
.discovery-panel {
  padding: 24px;
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 18px;
  background: rgba(8, 31, 42, 0.86);
  color: #e5eefc;
}

.heading {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-wrap: wrap;
  gap: 16px;
  margin-bottom: 24px;
}

.heading h2 {
  margin: 0 0 8px;
  color: #f8fafc;
}

.heading p {
  color: #cbd5e1;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.secondary-link {
  color: #67e8f9;
  text-decoration: none;
}

.search-label {
  display: block;
  margin-bottom: 8px;
  font-weight: 600;
}

.search-input {
  width: 100%;
  padding: 12px 14px;
  margin-bottom: 24px;
  border: 1px solid #475569;
  border-radius: 10px;
  background: #0f172a;
  color: #f8fafc;
}

.organization-grid {
  display: grid;
  grid-template-columns: repeat(
    auto-fit,
    minmax(min(100%, 280px), 1fr)
  );
  gap: 16px;
}

.organization-card {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  gap: 20px;
  padding: 20px;
  border: 1px solid rgba(45, 212, 191, 0.18);
  border-radius: 14px;
  background: #0f172a;
}

.organization-card h3 {
  margin: 6px 0;
  color: #f8fafc;
}

.slug {
  color: #94a3b8;
  font-size: 0.85rem;
}

.join-button {
  padding: 11px 14px;
  border: 0;
  border-radius: 10px;
  background: #2dd4bf;
  color: #042f2e;
  font-weight: 700;
  cursor: pointer;
}

.join-button:disabled {
  opacity: 0.5;
  cursor: wait;
}

.status {
  display: inline-block;
  align-self: flex-start;
  padding: 8px 12px;
  border-radius: 10px;
  font-size: 0.85rem;
  font-weight: 700;
}

.member {
  background: rgba(45, 212, 191, 0.14);
  color: #5eead4;
}

.pending {
  background: rgba(251, 191, 36, 0.14);
  color: #fcd34d;
}

.error-message {
  color: #fca5a5;
}

.success-message {
  color: #5eead4;
}

.empty-state {
  padding: 24px 0;
  color: #94a3b8;
}
</style>
