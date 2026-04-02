// src/plugins/axios-story-mock.ts
type MockResponse = {
  data?: any;
  status?: number;
  statusText?: string;
};

const axiosMock = {
  get: async (url: string) => {
    console.log('[Axios Mock GET]', url);
    return { data: { message: `Mock GET response for ${url}` }, status: 200, statusText: 'OK' } as MockResponse;
  },
  post: async (url: string, body: any) => {
    console.log('[Axios Mock POST]', url, body);
    return { data: { message: `Mock POST response for ${url}` }, status: 200, statusText: 'OK' } as MockResponse;
  },
  put: async (url: string, body: any) => {
    console.log('[Axios Mock PUT]', url, body);
    return { data: { message: `Mock PUT response for ${url}` }, status: 200, statusText: 'OK' } as MockResponse;
  },
  delete: async (url: string) => {
    console.log('[Axios Mock DELETE]', url);
    return { data: { message: `Mock DELETE response for ${url}` }, status: 200, statusText: 'OK' } as MockResponse;
  },
};

export default axiosMock;