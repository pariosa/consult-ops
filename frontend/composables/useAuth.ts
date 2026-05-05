import { navigateTo, useRouter } from 'nuxt/app';
import { computed, ref } from 'vue';

type AuthRole = 'admin' | 'consultant' | 'client';

type AuthUser = {
  id?: number;
  name?: string;
  email: string;
  role: AuthRole;
  token: string;
};

const authUser = ref<AuthUser | null>(null);

export const useAuth = () => {
  const router = useRouter();

  const isLoggedIn = computed(() => !!authUser.value?.token);
  const role = computed(() => authUser.value?.role || null);

  function restoreAuth() {
    if (!process.client) return;

    const stored = localStorage.getItem('auth_user');
    if (!stored) return;

    try {
      authUser.value = JSON.parse(stored);
    } catch {
      localStorage.removeItem('auth_user');
      authUser.value = null;
    }
  }

  function setAuth(user: AuthUser) {
    authUser.value = user;

    if (process.client) {
      localStorage.setItem('auth_user', JSON.stringify(user));
    }
  }

  function logout() {
    authUser.value = null;

    if (process.client) {
      localStorage.removeItem('auth_user');
    }

    return navigateTo('/');
  }

  function redirectForRole(userRole = authUser.value?.role) {
    if (userRole === 'admin') return router.push('/admin');
    if (userRole === 'client') return router.push('/client-portal');
    return router.push('/project-portal');
  }

  return {
    authUser,
    isLoggedIn,
    role,
    restoreAuth,
    setAuth,
    logout,
    redirectForRole,
  };
};
