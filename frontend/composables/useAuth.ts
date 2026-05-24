import { computed, ref } from 'vue';
import { useOrganizationOnboarding } from './useOrganizationOnboarding';
import { getPortalRoute } from '~/utils/authRedirect';
import { useApi } from './useApi';

type AuthRole = 'admin' | 'consultant' | 'client';

type AuthUser = {
  id?: number;
  name?: string;
  email: string;
  user_type?: string;
  role: AuthRole | string;
  portal?: string;
  token: string;
};

type LoginResponse = {
  token: string;
  user: {
    id?: number;
    name?: string;
    email: string;
    user_type?: string;
  };
};

type LoginResult = LoginResponse & {
  redirectTo: string;
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
  const { getMyOrganizations, setCurrentOrganization } =
    useOrganizationOnboarding();

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
  }): Promise<LoginResult> => {
    try {
      const data = await post<LoginResponse>('/api/auth/login', {
        email: payload.email,
        password: payload.password,
        remember_me: payload.remember_me ?? false,
      });

      const userRole = data.user.user_type || payload.userType || 'consultant';

      const user: AuthUser = {
        ...data.user,
        token: data.token,
        role: userRole,
        portal: userRole,
      };

      setAuth(user);

      const orgs = await getMyOrganizations();

      if (!orgs.length) {
        return {
          ...data,
          redirectTo: '/onboarding',
        };
      }

      if (orgs.length === 1) {
        await setCurrentOrganization(orgs[0].organization_id);

        return {
          ...data,
          redirectTo: getPortalRoute(userRole),
        };
      }

      return {
        ...data,
        redirectTo: '/workspace-select',
      };
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

    throw new AuthError(String(message), 'UNKNOWN', status);
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
