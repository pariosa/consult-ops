<script setup lang="ts">
defineProps<{
  members: any[];
}>();
</script>

<template>
  <div v-if="!members.length" class="empty-state">
    No organization members yet.
  </div>

  <div v-else class="member-table">
    <div class="member-row member-row--header">
      <span>User ID</span>
      <span>Role</span>
      <span>Status</span>
      <span>Joined</span>
    </div>

    <div v-for="member in members" :key="member.id" class="member-row">
      <span>#{{ member.user_id }}</span>
      <span>{{ member.role }}</span>
      <span>{{ member.status }}</span>
      <span>{{ member.created_at }}</span>
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

.member-table {
  display: grid;
  gap: 10px;
}

.member-row {
  align-items: center;
  background: rgba(8, 31, 42, 0.86);
  border: 1px solid rgba(45, 212, 191, 0.18);
  border-radius: 14px;
  color: #cbd5e1;
  display: grid;
  gap: 12px;
  grid-template-columns: 1fr 1fr 1fr 1.4fr;
  padding: 14px;
}

.member-row--header {
  background: rgba(2, 12, 23, 0.92);
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.1em;
  text-transform: uppercase;
}
</style>
