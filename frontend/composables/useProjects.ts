import { useApi } from '~/composables/useApi';

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

  // // CREATE project
  // const createProject = (payload: any) => {
  //   return post(`/api/projects`, payload);
  // };

  async function getOrganizationProjects(organizationId: number) {
    return await get(`/api/organizations/${organizationId}/projects`);
  }

  async function createProject(organizationId: number, payload: any) {
    return await post(`/api/organizations/${organizationId}/projects`, payload);
  }

  return {
    getProject,
    getProjects,
    createProject,
    getOrganizationProjects,
  };
};
