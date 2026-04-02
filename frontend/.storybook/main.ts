import { mergeConfig } from 'vite';
export default {
  stories: ['../components/**/*.stories.@(js|ts|vue)'],

  addons: [
    '@storybook/addon-links',
    '@storybook/addon-essentials',
    '@storybook/addon-interactions',
    '@chromatic-com/storybook'
  ],

  framework: {
    name: '@storybook/vue3-vite',
    options: {},
  },

  core: {
    builder: '@storybook/builder-vite'
  },

  viteFinal: async (config: any) => {
    // register the Vue plugin for Storybook
    return mergeConfig(config, {
      plugins: [require('@vitejs/plugin-vue')()],
    });
  },

  docs: {
    autodocs: true
  }
};