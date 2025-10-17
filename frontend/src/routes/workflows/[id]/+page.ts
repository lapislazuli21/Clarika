import { GraphQLClient } from 'graphql-request';
import type { PageLoad } from './$types';

const GQL_ENDPOINT = 'http://127.0.0.1:8080/graphql';
const client = new GraphQLClient(GQL_ENDPOINT);

const GET_TEMPLATE_DETAILS_QUERY = `
  query GetTemplateDetails($id: ID!) {
    getWorkflowTemplateById(id: $id) {
      id
      name
      description
      steps {
        id
        stepName
        stepOrder
        role
      }
    }
  }
`;

interface WorkflowStep {
  id: string;
  stepName: string;
  stepOrder: number;
  role: string | null;
}
interface WorkflowTemplate {
  id: string;
  name: string;
  description: string | null;
  steps: WorkflowStep[];
}
interface GetTemplateResponse {
  getWorkflowTemplateById: WorkflowTemplate | null;
}

export const load: PageLoad = async ({ params }) => {
  try {
    const variables = { id: params.id };
    const response = await client.request<GetTemplateResponse>(GET_TEMPLATE_DETAILS_QUERY, variables);
    return {
      template: response.getWorkflowTemplateById,
    };
  } catch (error) {
    console.error("Error fetching template details:", error);
    return { template: null, error: "Failed to load template." };
  }
};