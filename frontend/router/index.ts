// /router/index.ts
import {
  createRouter,
  createWebHistory,
  type RouteRecordRaw,
} from 'vue-router';

// Admin Views
import Dashboard from '@/views/Admin/Dashboard.vue';
import Clients from '@/views/Admin/Clients.vue';
import Projects from '@/views/Admin/Projects.vue';
import Contracts from '@/views/Admin/Contracts.vue';
import Invoices from '@/views/Admin/Invoices.vue';
import Payments from '@/views/Admin/Payments.vue';
import Home from '@/pages/index.vue';
// Optional: Auth pages
import AdminLogin from '@/views/Auth/AdminLogin.vue';
import ClientLogin from '@/views/Auth/ClientLogin.vue';
import ConsultantLogin from '~/views/Auth/ConsultantLogin.vue';
import MyWelcome from '~/components/MyWelcome.vue';

// Define routes
const routes: Array<RouteRecordRaw> = [
  // Home
  {
    path: '/home',
    name: 'home',
    component: MyWelcome,
  },
  // Auth
  {
    path: '/admin/login',
    name: 'AdminLogin',
    component: AdminLogin,
  },
  {
    path: '/consultant/login',
    name: 'ConsultantLogin',
    component: ConsultantLogin,
  },
  {
    path: '/clients/login',
    name: 'ClientLogin',
    component: ClientLogin,
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
        component: Clients,
      },
      {
        path: 'projects',
        name: 'Projects',
        component: Projects,
      },
      {
        path: 'contracts',
        name: 'Contracts',
        component: Contracts,
      },
      {
        path: 'invoices',
        name: 'Invoices',
        component: Invoices,
      },
      {
        path: 'payments',
        name: 'Payments',
        component: Payments,
      },
    ],
  },
  // Redirect root to admin dashboard
  {
    path: '/',
    redirect: '/admin',
  },
  // Catch-all 404
  {
    path: '/:pathMatch(.*)*',
    redirect: '/admin',
  },
];

// Create router
const router = createRouter({
  // history: createWebHistory(import.meta.env.BASE_URL),
  history: createWebHistory('localhost'),

  routes,
});

export default router;
