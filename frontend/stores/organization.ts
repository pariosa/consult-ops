// frontend/stores/organization.ts
import { defineStore } from 'pinia';
import { useApi } from '~/composables/useApi';

export type Organization = {
  id: string;
  name: string;
  created_at?: string;
};

export type OrganizationMember = {
  id: string;
  user_id: string;
  organization_id: string;
  email: string;
  name?: string;
  role: 'owner' | 'admin' | 'member' | 'client';
};

export const useOrganizationStore = defineStore('organization', {
  state: () => ({
    organization: null as Organization | null,
    members: [] as OrganizationMember[],
    loading: false,
    error: null as string | null,
  }),

  getters: {
    organizationId: (state) => state.organization?.id ?? null,
    memberCount: (state) => state.members.length,
  },

  actions: {
    async fetchCurrentOrganization() {
      const { apiFetch } = useApi();
      this.loading = true;
      this.error = null;

      try {
        this.organization = await apiFetch<Organization>(
          '/api/me/organization',
        );
      } catch (error: any) {
        this.error = error?.message || 'Failed to load organization';
      } finally {
        this.loading = false;
      }
    },

    async fetchMembers() {
      if (!this.organization?.id) {
        await this.fetchCurrentOrganization();
      }

      const organizationId = this.organization?.id;
      if (!organizationId) return;

      const { apiFetch } = useApi();

      this.members = await apiFetch<OrganizationMember[]>(
        `/api/organizations/${organizationId}/members`,
      );
    },
  },
});
