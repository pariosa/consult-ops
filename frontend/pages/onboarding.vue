<script setup lang="ts">
import { ref } from 'vue';
import { useOrganizationOnboarding } from '~/composables/useOrganizationOnboarding';

const orgName = ref('');
const loading = ref(false);
const error = ref('');

const { createOrganization } = useOrganizationOnboarding();

const submit = async () => {
  error.value = '';

  if (!orgName.value.trim()) {
    error.value = 'Organization name is required.';
    return;
  }

  loading.value = true;

  try {
    await createOrganization({ name: orgName.value.trim() });
    await navigateTo('/project-portal');
  } catch (err: any) {
    error.value = err?.data || err?.message || 'Unable to create organization.';
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <section class="auth-page">
    <div class="copy">
      <p class="eyebrow">Workspace Setup</p>
      <h1>Create your organization.</h1>
      <p>
        Every account needs an organization workspace before projects,
        agreements, milestones, and transactions can be managed.
      </p>
    </div>

    <form class="auth-form" @submit.prevent="submit">
      <div class="form-header">
        <p class="eyebrow">First workspace</p>
        <h2>Organization details</h2>
        <p class="subtitle">You will become the owner of this workspace.</p>
      </div>

      <div class="form-group">
        <label for="org-name">Organization name</label>
        <input
          id="org-name"
          v-model="orgName"
          type="text"
          placeholder="Atlas Studio"
          required
        />
      </div>

      <button type="submit" :disabled="loading">
        {{ loading ? 'Creating...' : 'Create organization' }}
      </button>

      <p v-if="error" class="error">{{ error }}</p>
    </form>
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

.copy p,
.subtitle {
  color: #a8bdd2;
  line-height: 1.7;
}

.auth-form {
  width: min(100%, 420px);
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 2rem;
  border: 1px solid rgba(80, 210, 170, 0.35);
  border-radius: 1.25rem;
  background: #08131f;
  color: #eef6ff;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

input {
  border: 1px solid rgba(148, 163, 184, 0.35);
  border-radius: 0.75rem;
  padding: 0.75rem 0.85rem;
  background: #0f1d2b;
  color: white;
}

button {
  border: 0;
  border-radius: 0.85rem;
  padding: 0.85rem;
  color: #041016;
  font-weight: 700;
  cursor: pointer;
  background: linear-gradient(135deg, #60a5fa, #34d399);
}

button:disabled {
  opacity: 0.65;
  cursor: not-allowed;
}

.error {
  color: #f87171;
}
</style>
