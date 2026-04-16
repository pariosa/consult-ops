import { mergeConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

export default {
  stories: ['../components/**/*.stories.@(js|ts|vue)'],

  addons: [
    '@storybook/addon-links',
    '@storybook/addon-essentials',
    '@storybook/addon-interactions',
    '@chromatic-com/storybook',
  ],

  framework: {
    name: '@storybook/vue3-vite',
    options: {},
  },

  core: {
    builder: '@storybook/builder-vite',
  },

  viteFinal: async (config: any) => {
    return mergeConfig(config, {
      plugins: [
        vue(), // ✅ important: proper Vue SFC compiler
      ],
    });
  },

  docs: {
    autodocs: true,
  },
};
