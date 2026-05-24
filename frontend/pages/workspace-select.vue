<script setup lang="ts">
import { onMounted, ref } from 'vue';
import {
  type MyOrganization,
  useOrganizationOnboarding,
} from '~/composables/useOrganizationOnboarding';

const organizations = ref<MyOrganization[]>([]);
const loading = ref(true);
const error = ref('');

const { getMyOrganizations, setCurrentOrganization } =
  useOrganizationOnboarding();

onMounted(async () => {
  try {
    organizations.value = await getMyOrganizations();

    if (!organizations.value.length) {
      await navigateTo('/onboarding');
    }
  } catch (err: any) {
    error.value = err?.data || err?.message || 'Unable to load workspaces.';
  } finally {
    loading.value = false;
  }
});

const choose = async (organization: MyOrganization) => {
  error.value = '';

  try {
    await setCurrentOrganization(organization.organization_id);
    await navigateTo('/project-portal');
  } catch (err: any) {
    error.value = err?.data || err?.message || 'Unable to select workspace.';
  }
};
</script>

<template>
  <section class="auth-page">
    <div class="copy">
      <p class="eyebrow">Choose Workspace</p>
      <h1>Select your organization.</h1>
      <p>
        Your account belongs to multiple workspaces. Choose where you want to
        work today.
      </p>
    </div>

    <div class="workspace-card">
      <p v-if="loading">Loading workspaces...</p>
      <p v-else-if="error" class="error">{{ error }}</p>

      <div v-else class="workspace-list">
        <button
          v-for="org in organizations"
          :key="org.organization_id"
          type="button"
          class="workspace-option"
          @click="choose(org)"
        >
          <strong>{{ org.name }}</strong>
          <span>{{ org.role }}</span>
        </button>
      </div>
    </div>
  </section>
</template>

<style scoped>
.auth-page {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 440px;
  gap: 4rem;
  align-items: center;
  max-width: 1120px;
  margin: 0 auto;
}

.copy {
  color: white;
}

.eyebrow {
  color: #55d6be;
  text-transform: uppercase;
  letter-spacing: 0.12em;
}

h1 {
  max-width: 680px;
  font-size: clamp(2.4rem, 6vw, 4.5rem);
  line-height: 0.95;
  margin: 0 0 1rem;
}

.copy p {
  color: #a8bdd2;
  line-height: 1.7;
}

.workspace-card {
  padding: 2rem;
  border-radius: 1.25rem;
  background: #08131f;
  color: #eef6ff;
  border: 1px solid rgba(80, 210, 170, 0.35);
}

.workspace-list {
  display: grid;
  gap: 0.85rem;
}

.workspace-option {
  text-align: left;
  border: 1px solid rgba(148, 163, 184, 0.35);
  border-radius: 1rem;
  padding: 1rem;
  background: #0f1d2b;
  color: white;
  cursor: pointer;
}

.workspace-option span {
  display: block;
  margin-top: 0.25rem;
  color: #9fb3c8;
}

.error {
  color: #f87171;
}
</style>
