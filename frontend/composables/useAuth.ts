import { computed, ref } from 'vue';
import { useApi } from './useApi';

type AuthRole = 'admin' | 'consultant' | 'client';

type AuthUser = {
  id?: number;
  name?: string;
  email: string;
  role: AuthRole;
  token: string;
};

type AuthErrorCode =
  | 'EMAIL_VERIFICATION_REQUIRED'
  | 'INVALID_CREDENTIALS'
  | 'RATE_LIMITED'
  | 'UNKNOWN';

export class AuthError extends Error {
  code: AuthErrorCode;
  status: number;

  constructor(message: string, code: AuthErrorCode, status: number) {
    super(message);
    this.name = 'AuthError';
    this.code = code;
    this.status = status;
  }
}

const authUser = ref<AuthUser | null>(null);
const hasRestoredAuth = ref(false);

export const useAuth = () => {
  const router = useRouter();
  const { post } = useApi();

  const isLoggedIn = computed(() => !!authUser.value?.token);
  const role = computed(() => authUser.value?.role || null);

  function restoreAuth() {
    if (!process.client || hasRestoredAuth.value) return;

    hasRestoredAuth.value = true;

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
  const login = async (payload: {
    email: string;
    password: string;
    remember_me?: boolean;
    userType?: string;
  }) => {
    try {
      const data = await post('/api/auth/login', {
        email: payload.email,
        password: payload.password,
        remember_me: payload.remember_me ?? false,
      });

      const user: AuthUser = {
        ...data.user,
        token: data.token,
        role: data.user.user_type || payload.userType || 'consultant',
      };

      setAuth(user);

      return data;
    } catch (err: any) {
      normalizeAuthError(err);
    }
  };
  const resendVerification = async (email: string) => {
    return await post<{ message: string }>('/api/auth/resend-verification', {
      email,
    });
  };
  function normalizeAuthError(err: any): never {
    const status = err?.status || err?.response?.status || 500;

    const message =
      err?.data?.message ||
      err?.data ||
      err?.message ||
      'Authentication request failed.';

    if (
      status === 403 &&
      typeof message === 'string' &&
      message.includes('Email verification required')
    ) {
      throw new AuthError(
        'Please verify your email before logging in.',
        'EMAIL_VERIFICATION_REQUIRED',
        status,
      );
    }

    if (status === 429) {
      throw new AuthError(
        'Too many failed login attempts. Please wait and try again.',
        'RATE_LIMITED',
        status,
      );
    }

    if (status === 401) {
      throw new AuthError(
        'Invalid email or password.',
        'INVALID_CREDENTIALS',
        status,
      );
    }

    throw new AuthError(message, 'UNKNOWN', status);
  }
  return {
    resendVerification,
    login,
    authUser,
    isLoggedIn,
    role,
    restoreAuth,
    setAuth,
    logout,
    redirectForRole,
  };
};
