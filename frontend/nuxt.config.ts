import { defineNuxtConfig } from 'nuxt/config';

export default defineNuxtConfig({
  ssr: false,
  compatibilityDate: '2024-04-01',
  modules: ['@pinia/nuxt'],

  css: ['~/assets/css/forms.css', '~/assets/css/theme.css'],
  vite: {
    ssr: {
      noExternal: ['nuxt'],
    },
  },
});
