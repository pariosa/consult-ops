<script setup lang="ts">
import { useOrganizationMembers } from '~/composables/useOrganizationMembers';

const route = useRoute();
const router = useRouter();
const { acceptInvitation } = useOrganizationMembers();

const loading = ref(false);
const error = ref('');
const success = ref('');
const token = computed(() => String(route.query.token || ''));

async function accept() {
  if (!token.value) {
    error.value = 'Missing invitation token.';
    return;
  }

  loading.value = true;
  error.value = '';
  success.value = '';

  try {
    await acceptInvitation(token.value);
    success.value = 'Invitation accepted. Redirecting...';

    setTimeout(() => {
      router.push('/organization/members');
    }, 800);
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to accept invitation.';
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <DashboardShell
    title="Accept Invitation"
    subtitle="Join an organization workspace in Consult Ops."
  >
    <section class="portal-section">
      <p class="eyebrow">Organization Invite</p>
      <h2>Accept your invitation</h2>
      <p>
        You must be signed in as the email address that received this
        invitation.
      </p>

      <button class="form-button" :disabled="loading || !token" @click="accept">
        {{ loading ? 'Accepting...' : 'Accept Invitation' }}
      </button>

      <div v-if="error" class="form-error">{{ error }}</div>
      <div v-if="success" class="success-state">{{ success }}</div>
    </section>
  </DashboardShell>
</template>

<style scoped>
.portal-section {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 18px;
  background: linear-gradient(
    180deg,
    rgba(15, 23, 42, 0.96),
    rgba(2, 12, 23, 0.96)
  );
  color: #e5eefc;
  padding: 24px;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 800;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

h2 {
  color: #f8fafc;
}

p {
  color: #cbd5e1;
}

.form-button {
  border: 0;
  border-radius: 12px;
  background: linear-gradient(90deg, #60a5fa, #34d399);
  color: #04111f;
  cursor: pointer;
  font-weight: 800;
  margin-top: 16px;
  padding: 12px 16px;
}

.form-error,
.success-state {
  border-radius: 14px;
  margin-top: 16px;
  padding: 14px;
}

.form-error {
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
}

.success-state {
  color: #6ee7b7;
}
</style>
