
import type { StorybookConfig } from '@storybook/vue3-vite';
import vue from '@vitejs/plugin-vue';
import { mergeConfig } from 'vite';
import { fileURLToPath, URL } from 'node:url';

const frontendRoot = fileURLToPath(new URL('../', import.meta.url));

const config: StorybookConfig = {
  stories: ['../components/**/*.stories.@(js|ts|vue)'],
  addons: [
    '@storybook/addon-links',
    '@storybook/addon-essentials',
    '@chromatic-com/storybook',
  ],
  framework: {
    name: '@storybook/vue3-vite',
    options: {},
  },
  async viteFinal(config) {
    return mergeConfig(config, {
      plugins: [vue()],
      esbuild: {
        tsconfigRaw: {
          compilerOptions: {
            target: 'esnext',
            jsx: 'preserve',
          },
        },
      },
      resolve: {
        alias: {
          '~': frontendRoot,
          '@': frontendRoot,
        },
      },
    });
  },
};

export default config;
