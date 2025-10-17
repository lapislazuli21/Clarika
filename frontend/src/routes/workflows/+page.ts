import { GraphQLClient } from 'graphql-request';
import type { PageLoad } from './$types';

const GQL_ENDPOINT = 'http://127.0.0.1:8080/graphql';
const client = new GraphQLClient(GQL_ENDPOINT);

const GET_TEMPLATES_QUERY = `
  query GetWorkflowTemplates {
    getWorkflowTemplates {
      id
      name
      description
    }
  }
`;

interface WorkflowTemplate {
  id: string;
  name: string;
  description: string | null;
}

interface GetTemplatesResponse {
  getWorkflowTemplates: WorkflowTemplate[];
}

export const load: PageLoad = async () => {
  try {
    const response = await client.request<GetTemplatesResponse>(GET_TEMPLATES_QUERY);
    return {
      templates: response.getWorkflowTemplates,
    };
  } catch (error) {
    console.error("Error fetching workflow templates:", error);
    return {
      templates: [],
      error: "Failed to load workflow templates.",
    };
  }
};