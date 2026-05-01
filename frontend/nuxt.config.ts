// nuxt.config.ts
import { defineNuxtConfig } from 'nuxt/config';

export default defineNuxtConfig({
  ssr: false,
  compatibilityDate: '2024-04-01', // Note: ensure this is a real date (2024, not 2026)
  experimental: {
    appManifest: false,
    externalVue: false,
    payloadExtraction: false,
  },
  devServer: {
    // This forces the dev server to use a standard port instead of trying to use a socket
    port: 3000,
  },
  vite: {
    // Use the old bundler to avoid the SocketPath bug
    // @ts-ignore
    devBundler: 'legacy',
  },
} as any);
