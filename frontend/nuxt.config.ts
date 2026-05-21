import { defineNuxtConfig } from 'nuxt/config';

export default defineNuxtConfig({
  ssr: false,
  compatibilityDate: '2024-04-01',
  modules: ['@pinia/nuxt'],
  runtimeConfig: {
    public: {
      apiBase: process.env.NUXT_PUBLIC_API_BASE || 'http://127.0.0.1:8000',
    },
  },
  css: ['~/assets/css/forms.css', '~/assets/css/theme.css'],
  vite: {
    ssr: {
      noExternal: ['nuxt'],
    },
  },
});
