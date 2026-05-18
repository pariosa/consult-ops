import { computed, ref } from 'vue';
import { useApi } from './useApi';

export function useNotifications() {
  const api = useApi();

  const notifications = ref<any[]>([]);
  const loading = ref(false);
  const error = ref('');

  const unreadCount = computed(
    () => notifications.value.filter((item) => !item.read_at).length,
  );

  async function refreshNotifications() {
    loading.value = true;
    error.value = '';

    try {
      notifications.value = await api.get('/api/notifications');
    } catch (err: any) {
      error.value = err?.message || 'Failed to load notifications.';
    } finally {
      loading.value = false;
    }
  }

  async function markRead(id: number) {
    await api.post(`/api/notifications/${id}/read`, {});
    await refreshNotifications();
  }

  async function markAllRead() {
    await api.post('/api/notifications/read-all', {});
    await refreshNotifications();
  }

  return {
    notifications,
    loading,
    error,
    unreadCount,
    refreshNotifications,
    markRead,
    markAllRead,
  };
}
