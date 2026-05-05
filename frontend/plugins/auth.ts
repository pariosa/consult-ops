import { defineNuxtPlugin } from 'nuxt/app';
import { useAuth } from '~/composables/useAuth';

export default defineNuxtPlugin(() => {
  const { restoreAuth } = useAuth();
  restoreAuth();
});
