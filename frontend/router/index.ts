// /router/index.ts
import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';

// Admin Views
import Dashboard from '@/views/Admin/Dashboard.vue';
import Clients from '@/views/Admin/Clients.vue';
import Projects from '@/views/Admin/Projects.vue';
import Contracts from '@/views/Admin/Contracts.vue';
import Invoices from '@/views/Admin/Invoices.vue';
import Payments from '@/views/Admin/Payments.vue';

// Optional: Auth pages
import Login from '@/views/Auth/Login.vue';
import Register from '@/views/Auth/Register.vue';

// Define routes
const routes: Array<RouteRecordRaw> = [
  // Auth
  {
    path: '/login',
    name: 'Login',
    component: Login
  },
  {
    path: '/register',
    name: 'Register',
    component: Register
  },
  // Admin Dashboard
  {
    path: '/admin',
    name: 'Dashboard',
    component: Dashboard,
    children: [
      {
        path: 'clients',
        name: 'Clients',
        component: Clients
      },
      {
        path: 'projects',
        name: 'Projects',
        component: Projects
      },
      {
        path: 'contracts',
        name: 'Contracts',
        component: Contracts
      },
      {
        path: 'invoices',
        name: 'Invoices',
        component: Invoices
      },
      {
        path: 'payments',
        name: 'Payments',
        component: Payments
      }
    ]
  },
  // Redirect root to admin dashboard
  {
    path: '/',
    redirect: '/admin'
  },
  // Catch-all 404
  {
    path: '/:pathMatch(.*)*',
    redirect: '/admin'
  }
];

// Create router
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes
});

export default router;