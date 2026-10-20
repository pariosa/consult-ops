import { useApi } from '../composables/useApi';


export interface Project {
  id: number;
  organization_id?: number;
  client_id: number;
  name: string;
  status?: string | null;
}

export const useProjects = () => {
  const { get, post } = useApi();

  // GET single project
  const getProject = (id: number) => {
    return get(`/api/projects/${id}`);
  };

  // GET all projects (optional, useful later)
  const getProjects = () => {
    return get(`/api/projects`);
  };

  async function getOrganizationProjects(
    organizationId: number,
  ): Promise<Project[]> {
    return get<Project[]>(
      `/api/organizations/${organizationId}/projects`,
    );
  }

  async function createProject(
    organizationId: number,
    payload: any,
  ): Promise<Project> {
    return post<Project>(
      `/api/organizations/${organizationId}/projects`,
      payload,
    );
  }

  return {
    getProject,
    getProjects,
    createProject,
    getOrganizationProjects,
  };
};
