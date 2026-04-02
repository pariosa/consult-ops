// src/stores/auth.ts
import { defineStore } from 'pinia';
import axios from 'axios';
import router from '@/router';

interface User {
  email: string;
  id: number;
}

interface AuthState {
  token: string | null;
  user: User | null;
}

export const useAuthStore = defineStore('auth', {
  state: (): AuthState => ({
    token: localStorage.getItem('token'),
    user: null,
  }),
  actions: {
    async login(email: string, password: string) {
      try {
        const res = await axios.post('/api/auth/login', { email, password });
        this.token = res.data.token;
        this.user = res.data.user;
        localStorage.setItem('token', this.token);
        axios.defaults.headers.common['Authorization'] = `Bearer ${this.token}`;
        router.push('/admin');
      } catch (err) {
        console.error(err);
        throw err;
      }
    },
    logout() {
      this.token = null;
      this.user = null;
      localStorage.removeItem('token');
      delete axios.defaults.headers.common['Authorization'];
      router.push('/login');
    },
    setUser(user: User) {
      this.user = user;
    }
  }
});