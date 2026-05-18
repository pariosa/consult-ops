export type UserType =
  | 'super_admin'
  | 'owner'
  | 'admin'
  | 'finance_admin'
  | 'operations_manager'
  | 'contractor'
  | 'client_viewer'
  | 'consultant'
  | 'client'
  | 'member';

const allOrgRoles: UserType[] = [
  'owner',
  'admin',
  'finance_admin',
  'operations_manager',
  'contractor',
  'client_viewer',
  'consultant',
  'client',
  'member',
];

const opsRoles: UserType[] = [
  'owner',
  'admin',
  'finance_admin',
  'operations_manager',
  'consultant',
];

const financeRoles: UserType[] = ['owner', 'admin', 'finance_admin'];
const adminRoles: UserType[] = ['owner', 'admin'];
const platformRoles: UserType[] = ['super_admin'];

export const navItems = [
  {
    label: 'Platform Admin',
    to: '/platform',
    roles: platformRoles,
  },
  {
    label: 'Admin Home',
    to: '/admin',
    roles: ['admin', 'owner', 'super_admin'],
  },
  {
    label: 'Users',
    to: '/admin/users',
    roles: ['admin', 'owner', 'super_admin'],
  },
  {
    label: 'Notifications',
    to: '/notifications',
    roles: ['super_admin', ...allOrgRoles],
  },
  {
    label: 'Organization HQ',
    to: '/organization',
    roles: ['super_admin', ...allOrgRoles],
  },
  {
    label: 'Members',
    to: '/organization/members',
    roles: ['super_admin', ...adminRoles],
  },
  {
    label: 'Invitations',
    to: '/organization/invitations',
    roles: ['super_admin', ...adminRoles],
  },
  {
    label: 'Engagements',
    to: '/engagements',
    roles: ['super_admin', ...allOrgRoles],
  },
  {
    label: 'Agreements',
    to: '/engagements/1/agreements',
    roles: ['super_admin', ...financeRoles],
  },
  {
    label: 'Operational Finance',
    to: '/organization/finance',
    roles: ['super_admin', ...financeRoles],
  },
  {
    label: 'Transactions',
    to: '/organization/transactions',
    roles: ['super_admin', ...financeRoles, 'operations_manager'],
  },
  {
    label: 'Projects',
    to: '/organization/projects',
    roles: ['super_admin', ...allOrgRoles],
  },
  {
    label: 'Clients',
    to: '/organization/clients',
    roles: ['super_admin', ...opsRoles, 'client_viewer'],
  },
  {
    label: 'Billing',
    to: '/settings/billing',
    roles: ['super_admin', ...financeRoles],
  },
  {
    label: 'Profile',
    to: '/profile',
    roles: ['super_admin', ...allOrgRoles],
  },
];
