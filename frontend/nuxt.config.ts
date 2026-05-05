import { defineNuxtConfig } from 'nuxt/config';

export default defineNuxtConfig({
  ssr: false,
  compatibilityDate: '2024-04-01',
  css: ['~/assets/css/forms.css'],
  vite: {
    ssr: {
      noExternal: ['nuxt'],
    },
  },
});
