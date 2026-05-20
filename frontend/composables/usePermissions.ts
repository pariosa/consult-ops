import { computed } from 'vue';
import { useAuth } from '~/composables/useAuth';

const agreementManagerRoles = [
  'super_admin',
  'owner',
  'admin',
  'finance_admin',
  'operations_manager',
  'payment_moderator',
];

function localAuthUser() {
  if (!process.client) return null;

  try {
    return JSON.parse(localStorage.getItem('auth:user') || 'null');
  } catch {
    return null;
  }
}

export function usePermissions() {
  const { authUser } = useAuth();

  const user = computed(() => authUser.value || localAuthUser());

  const role = computed(() => {
    return (
      user.value?.user_type || user.value?.role || user.value?.portal || ''
    );
  });

  const canManageFinance = computed(() =>
    ['owner', 'admin', 'finance_admin', 'super_admin'].includes(role.value),
  );

  const canManageAgreements = computed(() =>
    agreementManagerRoles.includes(role.value),
  );

  const canProcessTransactions = computed(() =>
    [
      'owner',
      'admin',
      'finance_admin',
      'operations_manager',
      'super_admin',
    ].includes(role.value),
  );

  return {
    role,
    canManageFinance,
    canManageAgreements,
    canProcessTransactions,
  };
}
