<script setup lang="ts">
import { useNotifications } from '~/composables/useNotifications';

const {
  notifications,
  loading,
  error,
  unreadCount,
  refreshNotifications,
  markRead,
  markAllRead,
} = useNotifications();

onMounted(async () => {
  try {
    await refreshNotifications();
  } catch (err) {
    console.error(err);
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <DashboardShell
    title="Notifications"
    subtitle="Operational updates, workflow events, and pending actions."
  >
    <h1 class="sr-only">Notifications</h1>

    <section class="portal-section">
      <div class="section-header">
        <div>
          <p class="eyebrow">Notification Center</p>
          <h2>{{ unreadCount }} unread</h2>
        </div>

        <button
          v-if="unreadCount"
          class="form-button secondary"
          @click="markAllRead"
        >
          Mark all read
        </button>
      </div>

      <p v-if="loading">Loading notifications...</p>
      <p v-else-if="error" class="form-error">{{ error }}</p>

      <div v-else-if="!notifications.length" class="empty-state">
        No notifications yet.
      </div>

      <div v-else class="notification-list">
        <article
          v-for="item in notifications"
          :key="item.id"
          class="notification-card"
          :class="{ unread: !item.read_at }"
        >
          <div>
            <p class="eyebrow">{{ item.notification_type }}</p>
            <h3>{{ item.title }}</h3>
            <p>{{ item.body }}</p>
            <small>{{ item.created_at }}</small>
          </div>
          <div class="notification-actions">
            <button
              v-if="!item.read_at"
              class="form-button secondary"
              @click="markRead(item.id)"
            >
              Mark read
            </button>
          </div>
        </article>
      </div>
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

.section-header {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  align-items: center;
  margin-bottom: 20px;
}

.notification-actions {
  display: flex;
  justify-content: flex-end;
}

.notification-actions .form-button {
  width: auto;
  min-width: 120px;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.12em;
  margin: 0 0 8px;
  text-transform: uppercase;
}

h2,
h3 {
  color: #f8fafc;
  margin: 0 0 8px;
}

.notification-list {
  display: grid;
  gap: 12px;
}

.notification-card {
  border: 1px solid rgba(45, 212, 191, 0.16);
  border-radius: 16px;
  background: rgba(8, 31, 42, 0.72);
  display: flex;
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 18px;
}
.notification-actions {
  display: flex;
  justify-content: flex-end;
}

.notification-card.unread {
  border-color: rgba(96, 165, 250, 0.5);
  background: rgba(30, 64, 175, 0.16);
}

.form-button {
  border: 0;
  border-radius: 12px;
  cursor: pointer;
  font-weight: 800;
  padding: 10px 14px;
}

.form-button.secondary {
  background: rgba(15, 23, 42, 0.95);
  border: 1px solid rgba(45, 212, 191, 0.32);
  color: #e5eefc;
}

.empty-state,
.form-error {
  border-radius: 14px;
  padding: 16px;
}

.empty-state {
  border: 1px dashed rgba(45, 212, 191, 0.28);
  color: #cbd5e1;
}

.form-error {
  background: rgba(127, 29, 29, 0.24);
  color: #fecaca;
}
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}
</style>
