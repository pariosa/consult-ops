// frontend/utils/navItems.ts

export type UserType = 'admin' | 'consultant' | 'client';

export const navItems = [
  {
    label: 'Admin Home',
    to: '/admin',
    roles: ['admin'],
  },
  {
    label: 'Users',
    to: '/admin/users',
    roles: ['admin'],
  },
  {
    label: 'Organization',
    to: '/organization',
    roles: ['admin', 'consultant', 'client'],
  },
  {
    label: 'Engagements',
    to: '/engagements',
    roles: ['admin', 'consultant', 'client'],
  },
  {
    label: 'Members',
    to: '/organization/members',
    roles: ['admin', 'consultant'],
  },
  {
    label: 'Clients',
    to: '/clients',
    roles: ['admin', 'consultant'],
  },
  {
    label: 'Projects',
    to: '/organization/projects',
    roles: ['admin', 'consultant', 'client'],
  },
  {
    label: 'Billing',
    to: '/settings/billing',
    roles: ['admin', 'consultant'],
  },
  {
    label: 'Profile',
    to: '/profile',
    roles: ['admin', 'consultant', 'client'],
  },
];
