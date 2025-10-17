<script lang="ts">
  import { GraphQLClient } from "graphql-request";

  // --- Configuration ---
  const GQL_ENDPOINT = "http://127.0.0.1:8080/graphql";
  const client = new GraphQLClient(GQL_ENDPOINT);

  // --- Type Definitions ---
  interface ScopeProjectResponse {
    scopeProjectWithAi: string; // The API returns a JSON string
  }

  // --- Component State ---
  let projectDescription = "";
  let generatedTasks: string[] = [];
  let statusMessage = "";
  let isLoading = false;

  // --- GraphQL Mutation ---
  const SCOPE_PROJECT_MUTATION = `
    mutation ScopeProject($desc: String!) {
      scopeProjectWithAi(projectDescription: $desc)
    }
  `;

  // --- Form Submission Logic ---
  async function handleSubmit() {
    isLoading = true;
    statusMessage = "🤖 Scoping project with AI...";
    generatedTasks = [];

    try {
      const variables = { desc: projectDescription };
      const response = await client.request<ScopeProjectResponse>(
        SCOPE_PROJECT_MUTATION,
        variables
      );

      // 1. Get the raw string from the AI response.
      const rawResponse = response.scopeProjectWithAi;

      // 2. Use a regular expression to find the content between [ and ].
      // The 's' flag allows '.' to match newline characters.
      const jsonMatch = rawResponse.match(/(\[.*\])/s);

      // 3. If a match is found, parse it. Otherwise, assume the raw response is the JSON.
      if (jsonMatch && jsonMatch[0]) {
        const tasks = JSON.parse(jsonMatch[0]);
        generatedTasks = tasks;
      } else {
        // Fallback for cases where the AI returns only the clean array
        const tasks = JSON.parse(rawResponse);
        generatedTasks = tasks;
      }

      statusMessage = "✅ Tasks generated successfully!";
    } catch (error) {
      console.error("AI Scoping Error:", error);
      statusMessage =
        "❌ Failed to generate tasks. Please check the format of the AI response.";
    } finally {
      isLoading = false;
    }
  }
</script>

<main>
  <h1>AI Project Scoper</h1>
  <p>
    Describe your project goal, and the AI will generate a starting task list
    for you.
  </p>

  <form on:submit|preventDefault={handleSubmit}>
    <div class="form-group">
      <label for="description">Project Description:</label>
      <textarea
        id="description"
        bind:value={projectDescription}
        required
        rows="4"
        placeholder="e.g., Build a web app for users to share and discover new recipes."
      ></textarea>
    </div>

    <button type="submit" disabled={isLoading}>
      {#if isLoading}
        Generating...
      {:else}
        Generate Tasks
      {/if}
    </button>
  </form>

  {#if statusMessage}
    <p class="status">{statusMessage}</p>
  {/if}

  {#if generatedTasks.length > 0}
    <div class="results">
      <h2>Suggested Tasks:</h2>
      <ul>
        {#each generatedTasks as task}
          <li>{task}</li>
        {/each}
      </ul>
    </div>
  {/if}
</main>

<style>
  main {
    max-width: 700px;
    margin: 2rem auto;
    padding: 2rem;
    border: 1px solid #ccc;
    border-radius: 8px;
    font-family: sans-serif;
  }
  .form-group {
    margin-bottom: 1.5rem;
  }
  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: bold;
  }
  textarea {
    width: 100%;
    padding: 0.5rem;
    box-sizing: border-box;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-family: sans-serif;
  }
  button {
    width: 100%;
    padding: 0.75rem;
    background-color: #563d7c; /* A nice purple */
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
  }
  button:disabled {
    background-color: #aaa;
  }
  .status {
    margin-top: 1rem;
    text-align: center;
    font-weight: bold;
  }
  .results {
    margin-top: 2rem;
    padding: 1rem;
    background-color: #f9f9f9;
    border: 1px solid #eee;
    border-radius: 8px;
  }
  ul {
    padding-left: 20px;
  }
  li {
    margin-bottom: 0.5rem;
  }
</style>
