// frontend/composables/useApi.ts

export const useApi = () => {
  const apiFetch = async <T>(path: string, options: any = {}) => {
    const authUser = process.client
      ? JSON.parse(localStorage.getItem('auth_user') || 'null')
      : null;

    return await $fetch<T>(path, {
      baseURL: 'http://127.0.0.1:8000',
      ...options,
      headers: {
        ...(options.headers || {}),
        ...(authUser?.token
          ? { Authorization: `Bearer ${authUser.token}` }
          : {}),
      },
    });
  };

  return { apiFetch };
};
