import { GraphQLClient } from 'graphql-request';
import type { PageLoad } from './$types';

const GQL_ENDPOINT = 'http://127.0.0.1:8080/graphql';
const client = new GraphQLClient(GQL_ENDPOINT);

const GET_PROJECT_DETAILS_QUERY = `
  query GetProjectDetails($id: ID!) {
    getProjectById(id: $id) {
      id
      name
      description
      deadline
      tasks {
        id
        title
        status
        jiraTicketId
        raciAssignments {
          role
          user {
            email
          }
        }
      }
    }
  }
`;

const GET_USERS_QUERY = `
  query GetUsers {
    getUsers {
      id
      email
    }
  }
`;

const GET_TEMPLATES_QUERY = `
  query GetWorkflowTemplates {
    getWorkflowTemplates {
      id
      name
    }
  }
`;

// Define types for our data
interface Task {
  id: string;
  title: string;
  status: string;
  raciAssignments: RaciAssignment[];
  jiraTicketId: string | null;
}

interface Project {
  id: string;
  name: string;
  description: string | null;
  deadline: string | null;
  tasks: Task[];
}

interface GetProjectDetailsResponse {
  getProjectById: Project | null;
}

interface User {
  id: string;
  email: string;
}

interface GetUsersResponse {
  getUsers: User[];
}

interface RaciAssignment {
  role: string;
  user: {
    email: string;
  };
}

interface WorkflowTemplate {
  id: string;
  name: string;
}
interface GetTemplatesResponse {
  getWorkflowTemplates: WorkflowTemplate[];
}

// The 'params' object contains the dynamic parts of the URL, like our [id]
export const load: PageLoad = async ({ params }) => {
  try {
    const projectVariables = { id: params.id };
    const [projectResponse, usersResponse, templatesResponse] = await Promise.all([
      client.request<GetProjectDetailsResponse>(GET_PROJECT_DETAILS_QUERY, projectVariables),
      client.request<GetUsersResponse>(GET_USERS_QUERY),
      client.request<GetTemplatesResponse>(GET_TEMPLATES_QUERY)
    ]);

    return {
      project: projectResponse.getProjectById,
      users: usersResponse.getUsers,
      templates: templatesResponse.getWorkflowTemplates,
    };
  } catch (error) {
    console.error("Error fetching page data:", error);
    return {
      project: null,
      users: [],
      error: "Failed to load project page.",
    };
  }
};