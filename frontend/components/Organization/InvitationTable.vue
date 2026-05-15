<script setup lang="ts">
import InvitationStatusBadge from './InvitationStatusBadge.vue';

defineProps<{
  invitations: any[];
}>();

async function copyLink(inviteUrl?: string) {
  if (!inviteUrl) return;
  await navigator.clipboard.writeText(inviteUrl);
}
</script>

<template>
  <div v-if="!invitations.length" class="empty-state">
    No invitations have been sent yet.
  </div>

  <div v-else class="invite-table">
    <div class="invite-row invite-row--header">
      <span>Email</span>
      <span>Role</span>
      <span>Status</span>
      <span>Expires</span>
      <span>Actions</span>
    </div>

    <div v-for="invite in invitations" :key="invite.id" class="invite-row">
      <span>{{ invite.email }}</span>
      <span>{{ invite.role }}</span>
      <span><InvitationStatusBadge :status="invite.status" /></span>
      <span>{{ invite.expires_at }}</span>
      <span>
        <button
          v-if="invite.invite_url"
          class="mini-button"
          @click="copyLink(invite.invite_url)"
        >
          Copy Link
        </button>
      </span>
    </div>
  </div>
</template>

<style scoped>
.empty-state {
  border: 1px dashed rgba(45, 212, 191, 0.28);
  border-radius: 14px;
  color: #cbd5e1;
  padding: 18px;
}

.invite-table {
  display: grid;
  gap: 10px;
}

.invite-row {
  align-items: center;
  background: rgba(8, 31, 42, 0.86);
  border: 1px solid rgba(45, 212, 191, 0.18);
  border-radius: 14px;
  color: #cbd5e1;
  display: grid;
  gap: 12px;
  grid-template-columns: 1.4fr 0.8fr 0.8fr 1.4fr 0.8fr;
  padding: 14px;
}

.invite-row--header {
  background: rgba(2, 12, 23, 0.92);
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.1em;
  text-transform: uppercase;
}

.mini-button {
  background: rgba(15, 23, 42, 0.95);
  border: 1px solid rgba(45, 212, 191, 0.32);
  border-radius: 10px;
  color: #e5eefc;
  cursor: pointer;
  font-weight: 800;
  padding: 8px 10px;
}

@media (max-width: 800px) {
  .invite-row {
    grid-template-columns: 1fr;
  }

  .invite-row--header {
    display: none;
  }
}
</style>
