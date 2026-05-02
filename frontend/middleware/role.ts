import { defineNuxtRouteMiddleware, navigateTo } from 'nuxt/app';

// frontend/middleware/role.ts
export default defineNuxtRouteMiddleware((to) => {
  if (process.server) return;

  const raw = localStorage.getItem('auth_user');

  if (!raw) {
    return navigateTo('/consultant-login');
  }

  const user = JSON.parse(raw);
  const allowedUserTypes = to.meta.allowedUserTypes as string[] | undefined;

  if (allowedUserTypes && !allowedUserTypes.includes(user.user_type)) {
    return navigateTo('/unauthorized');
  }
});
