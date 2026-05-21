import { defineConfig } from 'vitest/config';
import vue from '@vitejs/plugin-vue';

export default defineConfig({
  plugins: [vue()],
  test: {
    environment: 'happy-dom',
    globals: true,
    exclude: [
      'node_modules',
      'dist',
      '.nuxt',
      'tests/e2e/**',
      '**/*.spec.e2e.ts',
    ],
  },

  resolve: {
    alias: {
      '~': new URL('./', import.meta.url).pathname,
      '@': new URL('./', import.meta.url).pathname,
    },
  },
});
