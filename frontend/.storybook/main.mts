import type { StorybookConfig } from '@storybook/vue3-vite';
import vue from '@vitejs/plugin-vue';
import { mergeConfig } from 'vite';

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
      plugins: [vue()], // This fixes the "invalid JS syntax" error for .vue files
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
          '~': '/Users/w0xy/code/consult-ops/frontend',
          '@': '/Users/w0xy/code/consult-ops/frontend',
        },
      },
    });
  },
};

export default config;
