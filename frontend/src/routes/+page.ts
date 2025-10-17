import { GraphQLClient } from 'graphql-request';
import type { PageLoad } from './$types';

// --- Configuration ---
const GQL_ENDPOINT = 'http://127.0.0.1:8080/graphql';
const client = new GraphQLClient(GQL_ENDPOINT);

// --- The GraphQL Query ---
const HEALTH_QUERY = `
  query HealthCheck {
    health
  }
`;

// --- Type Definition for the Response ---
interface HealthCheckResponse {
  health: string;
}

// --- SvelteKit load function ---
export const load: PageLoad = async () => {
  try {
    const response = await client.request<HealthCheckResponse>(HEALTH_QUERY);
    return {
      status: 'ok',
      message: response.health,
    };
  } catch (error) {
    console.error("GraphQL Health Check Error:", error);
    return {
      status: 'error',
      message: 'Failed to connect to the backend.',
    };
  }
};