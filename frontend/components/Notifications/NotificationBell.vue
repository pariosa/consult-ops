<script setup lang="ts">
import { computed, ref } from 'vue';

type NotificationItem = {
  id: number;
  organization_id: number;
  user_id?: number | null;
  recipient_email?: string | null;
  notification_type: string;
  title: string;
  body: string;
  entity_type?: string | null;
  entity_id?: number | null;
  read_at?: string | null;
  created_at: string;
};

const config = useRuntimeConfig();

const notifications = ref<NotificationItem[]>([]);
const loading = ref(false);
const open = ref(false);
const error = ref('');

const unreadCount = computed(
  () => notifications.value.filter((item) => !item.read_at).length,
);

function authHeaders() {
  if (!process.client) return {};

  const rawUser = localStorage.getItem('auth_user');
  const parsedUser = rawUser ? JSON.parse(rawUser) : null;

  const token =
    localStorage.getItem('auth_token') ||
    parsedUser?.token ||
    localStorage.getItem('token');

  return token
    ? {
        Authorization: `Bearer ${token}`,
      }
    : {};
}

async function loadNotifications() {
  loading.value = true;
  error.value = '';

  try {
    notifications.value = await $fetch<NotificationItem[]>(
      `${config.public.apiBase}/api/notifications`,
      {
        headers: authHeaders(),
      },
    );
  } catch (err: any) {
    error.value =
      err?.data?.error || err?.message || 'Failed to load notifications.';
  } finally {
    loading.value = false;
  }
}

async function markRead(notification: NotificationItem) {
  if (notification.read_at) return;

  try {
    await $fetch(
      `${config.public.apiBase}/api/notifications/${notification.id}/read`,
      {
        method: 'POST',
        headers: authHeaders(),
      },
    );

    notification.read_at = new Date().toISOString();
  } catch (err) {
    console.error('Failed to mark notification read', err);
  }
}

async function markAllRead() {
  try {
    await $fetch(`${config.public.apiBase}/api/notifications/read-all`, {
      method: 'POST',
      headers: authHeaders(),
    });

    const now = new Date().toISOString();

    notifications.value = notifications.value.map((item) => ({
      ...item,
      read_at: item.read_at || now,
    }));
  } catch (err) {
    console.error('Failed to mark all notifications read', err);
  }
}

function notificationTarget(notification: NotificationItem) {
  if (!notification.entity_type || !notification.entity_id) return null;

  if (notification.entity_type === 'engagement') {
    return `/engagements/${notification.entity_id}`;
  }

  if (notification.entity_type === 'milestone') {
    return `/engagements/milestones/${notification.entity_id}`;
  }

  if (notification.entity_type === 'organization') {
    return `/organization`;
  }

  if (notification.entity_type === 'transaction') {
    return `/operational-transactions/${notification.entity_id}`;
  }

  return null;
}

async function openNotification(notification: NotificationItem) {
  await markRead(notification);

  const target = notificationTarget(notification);

  if (target) {
    open.value = false;
    await navigateTo(target);
  }
}

function formatDate(value: string) {
  return new Date(value).toLocaleString();
}

onMounted(loadNotifications);
</script>

<template>
  <div class="notification-root">
    <button
      class="notification-button"
      type="button"
      aria-label="Notifications"
      @click="open = !open"
    >
      <span class="bell">🔔</span>

      <span v-if="unreadCount" class="notification-badge">
        {{ unreadCount }}
      </span>
    </button>

    <div v-if="open" class="notification-panel">
      <div class="notification-header">
        <div>
          <p class="eyebrow">Notifications</p>
          <h3>Recent Activity</h3>
        </div>

        <button
          v-if="unreadCount"
          class="mark-all-button"
          type="button"
          @click="markAllRead"
        >
          Mark all read
        </button>
      </div>

      <div v-if="loading" class="notification-empty">
        Loading notifications...
      </div>

      <div v-else-if="error" class="notification-error">
        {{ error }}
      </div>

      <div v-else-if="!notifications.length" class="notification-empty">
        No notifications yet.
      </div>

      <div v-else class="notification-list">
        <button
          v-for="notification in notifications"
          :key="notification.id"
          class="notification-item"
          :class="{ unread: !notification.read_at }"
          type="button"
          @click="openNotification(notification)"
        >
          <div class="notification-item-header">
            <strong>{{ notification.title }}</strong>
            <span v-if="!notification.read_at" class="unread-dot" />
          </div>

          <p>{{ notification.body }}</p>

          <small>
            {{ formatDate(notification.created_at) }}
          </small>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.notification-root {
  position: relative;
}

.notification-button {
  align-items: center;
  background: rgba(8, 31, 42, 0.88);
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 999px;
  color: #e5eefc;
  cursor: pointer;
  display: inline-flex;
  height: 42px;
  justify-content: center;
  position: relative;
  width: 42px;
}

.bell {
  font-size: 1rem;
}

.notification-badge {
  align-items: center;
  background: #fb7185;
  border-radius: 999px;
  color: #fff;
  display: inline-flex;
  font-size: 0.68rem;
  font-weight: 900;
  height: 20px;
  justify-content: center;
  min-width: 20px;
  padding: 0 6px;
  position: absolute;
  right: -6px;
  top: -6px;
}

.notification-panel {
  background: rgba(2, 12, 23, 0.98);
  border: 1px solid rgba(45, 212, 191, 0.24);
  border-radius: 18px;
  box-shadow: 0 20px 70px rgba(0, 0, 0, 0.35);
  color: #e5eefc;
  max-height: 520px;
  overflow: hidden;
  position: absolute;
  right: 0;
  top: 52px;
  width: min(420px, calc(100vw - 32px));
  z-index: 50;
}

.notification-header {
  align-items: center;
  border-bottom: 1px solid rgba(45, 212, 191, 0.18);
  display: flex;
  justify-content: space-between;
  gap: 16px;
  padding: 18px;
}

.notification-header h3 {
  color: #f8fafc;
  margin: 4px 0 0;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.68rem;
  font-weight: 900;
  letter-spacing: 0.12em;
  margin: 0;
  text-transform: uppercase;
}

.mark-all-button {
  background: rgba(45, 212, 191, 0.14);
  border: 1px solid rgba(45, 212, 191, 0.28);
  border-radius: 999px;
  color: #67e8f9;
  cursor: pointer;
  font-size: 0.76rem;
  font-weight: 800;
  padding: 8px 10px;
}

.notification-list {
  max-height: 420px;
  overflow-y: auto;
  padding: 8px;
}

.notification-item {
  background: transparent;
  border: 1px solid transparent;
  border-radius: 14px;
  color: inherit;
  cursor: pointer;
  display: block;
  padding: 14px;
  text-align: left;
  width: 100%;
}

.notification-item:hover,
.notification-item.unread {
  background: rgba(8, 31, 42, 0.88);
  border-color: rgba(45, 212, 191, 0.18);
}

.notification-item-header {
  align-items: center;
  display: flex;
  justify-content: space-between;
  gap: 12px;
}

.notification-item strong {
  color: #f8fafc;
}

.notification-item p {
  color: #cbd5e1;
  margin: 6px 0;
}

.notification-item small {
  color: #94a3b8;
}

.unread-dot {
  background: #67e8f9;
  border-radius: 999px;
  height: 9px;
  min-width: 9px;
}

.notification-empty,
.notification-error {
  color: #cbd5e1;
  padding: 18px;
}

.notification-error {
  color: #fecaca;
}
</style>
