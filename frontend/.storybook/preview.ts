// .storybook/preview.ts
import type { Preview } from '@storybook/vue3';

const axiosMock = {
  get: async (url: string) => ({ data: { message: `Mock GET ${url}` }, status: 200 }),
  post: async (url: string, body: any) => ({ data: { message: `Mock POST ${url}` }, status: 200 }),
  put: async (url: string, body: any) => ({ data: { message: `Mock PUT ${url}` }, status: 200 }),
  delete: async (url: string) => ({ data: { message: `Mock DELETE ${url}` }, status: 200 }),
};
export const decorators = [
  (story: () => any) => {
    return {
      components: { Story: story() },
      provide: { $axios: axiosMock },
      template: '<story />',
    };
  },
];

const preview: Preview = {
  decorators: decorators,
};

export default preview;