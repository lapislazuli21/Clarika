import { GraphQLClient } from 'graphql-request';
import type { PageLoad } from './$types';

const GQL_ENDPOINT = 'http://127.0.0.1:8080/graphql';
const client = new GraphQLClient(GQL_ENDPOINT);

const GET_PROJECTS_QUERY = `
  query GetProjects {
    getProjects {
      id
      name
      description
      deadline
    }
  }
`;

interface Project {
  id: string;
  name: string;
  description: string | null;
  deadline: string | null;
}

interface GetProjectsResponse {
  getProjects: Project[];
}

export const load: PageLoad = async () => {
  try {
    const response = await client.request<GetProjectsResponse>(GET_PROJECTS_QUERY);
    return {
      projects: response.getProjects,
    };
  } catch (error) {
    console.error("Error fetching projects:", error);
    return {
      projects: [],
      error: "Failed to load projects.",
    };
  }
};