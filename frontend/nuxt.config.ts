import { defineNuxtConfig } from 'nuxt/config';

export default defineNuxtConfig({
  ssr: false,
  compatibilityDate: '2024-04-01',
  vite: {
    ssr: {
      noExternal: ['nuxt'],
    },
  },
});
