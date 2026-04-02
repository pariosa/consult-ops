// src/plugins/axios.ts
import axios from 'axios';
import { defineNuxtPlugin } from '#app';

export default defineNuxtPlugin((nuxtApp) => {
  const api = axios.create({
    baseURL: 'http://127.0.0.1:8000',
  });

  // Only run in the browser
  if (process.client) {
    const token = localStorage.getItem('token');
    if (token) {
      api.defaults.headers.common['Authorization'] = `Bearer ${token}`;
    }
  }

  // Make it available globally via nuxtApp
  nuxtApp.provide('api', api);
});
