import { defineNuxtRouteMiddleware, navigateTo } from 'nuxt/app';

// frontend/middleware/role.ts
export default defineNuxtRouteMiddleware((to) => {
  if (process.server) return;

  const allowedRoles = to.meta.roles as string[] | undefined;
  if (!allowedRoles?.length) return;

  const raw = localStorage.getItem('auth_user');
  const user = raw ? JSON.parse(raw) : null;

  if (!user) {
    return navigateTo('/admin-login');
  }

  const userType = user.user_type || user.role || user.portal;

  if (userType === 'super_admin') {
    return;
  }

  if (!allowedRoles.includes(userType)) {
    return navigateTo('/unauthorized');
  }
});
