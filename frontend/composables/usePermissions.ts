import { computed } from 'vue';

export function usePermissions() {
  const user = useState<any>('auth:user');

  const role = computed(() => user.value?.user_type || user.value?.role || '');

  const canManageFinance = computed(() =>
    ['owner', 'admin', 'finance_admin'].includes(role.value),
  );

  const canManageAgreements = computed(() =>
    ['owner', 'admin', 'finance_admin'].includes(role.value),
  );

  const canProcessTransactions = computed(() =>
    ['owner', 'admin', 'finance_admin', 'operations_manager'].includes(
      role.value,
    ),
  );

  return {
    role,
    canManageFinance,
    canManageAgreements,
    canProcessTransactions,
  };
}
