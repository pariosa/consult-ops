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

  // ✅ GET
  const get = <T>(path: string, options: any = {}) =>
    apiFetch<T>(path, {
      method: 'GET',
      ...options,
    });

  // ✅ POST (what you asked for)
  const post = <T>(path: string, body: any = {}, options: any = {}) =>
    apiFetch<T>(path, {
      method: 'POST',
      body,
      ...options,
    });

  // ✅ PATCH
  const patch = <T>(path: string, body: any = {}, options: any = {}) =>
    apiFetch<T>(path, {
      method: 'PATCH',
      body,
      ...options,
    });

  // ✅ DELETE
  const del = <T>(path: string, options: any = {}) =>
    apiFetch<T>(path, {
      method: 'DELETE',
      ...options,
    });

  return {
    apiFetch,
    get,
    post,
    patch,
    delete: del, // avoid reserved word issues
  };
};
