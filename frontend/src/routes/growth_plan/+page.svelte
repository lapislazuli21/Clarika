<script lang="ts">
  import { GraphQLClient } from "graphql-request";

  // --- Configuration ---
  const GQL_ENDPOINT = "http://127.0.0.1:8080/graphql";
  const client = new GraphQLClient(GQL_ENDPOINT);

  // --- Type Definition for the GraphQL Response ---
  interface CreateGrowthTemplateResponse {
    createGrowthTemplate: {
      id: string;
      userId: string;
    };
  }

  // --- Form State ---
  let coreCompetencies = "";
  let developingSkills = "";
  let recentAchievements = "";
  let howToContribute = "";
  let statusMessage = "";
  let isLoading = false;

  // --- GraphQL Mutation ---
  const CREATE_TEMPLATE_MUTATION = `
    mutation CreateTemplate(
      $core: String!, 
      $dev: String!, 
      $achieve: String!, 
      $contrib: String!
    ) {
      createGrowthTemplate(
        coreCompetencies: $core
        developingSkills: $dev
        recentAchievements: $achieve
        howToContribute: $contrib
      ) {
        id
        userId
      }
    }
  `;

  // --- Form Submission Logic ---
  async function handleSubmit() {
    isLoading = true;
    statusMessage = "Submitting...";

    try {
      const variables = {
        core: coreCompetencies,
        dev: developingSkills,
        achieve: recentAchievements,
        contrib: howToContribute,
      };

      await client.request<CreateGrowthTemplateResponse>(
        CREATE_TEMPLATE_MUTATION,
        variables
      );

      statusMessage = `Success! Your growth plan has been submitted.`;
      // Clear the form on success
      coreCompetencies = "";
      developingSkills = "";
      recentAchievements = "";
      howToContribute = "";
    } catch (error) {
      console.error("Submission Error:", error);
      statusMessage = "Submission failed. Please try again.";
    } finally {
      isLoading = false;
    }
  }
</script>

<main>
  <h1>My Growth Plan</h1>
  <p>
    Update this quarterly to help us align your growth with company
    opportunities.
  </p>

  <form on:submit|preventDefault={handleSubmit}>
    <div class="form-group">
      <label for="core">Core Competencies (My Strengths):</label>
      <textarea id="core" bind:value={coreCompetencies} required rows="3"
      ></textarea>
    </div>

    <div class="form-group">
      <label for="dev">Developing Skills (What I'm Learning):</label>
      <textarea id="dev" bind:value={developingSkills} required rows="3"
      ></textarea>
    </div>

    <div class="form-group">
      <label for="achieve">Recent Achievements:</label>
      <textarea id="achieve" bind:value={recentAchievements} required rows="3"
      ></textarea>
    </div>

    <div class="form-group">
      <label for="contrib">How I Want to Contribute Next:</label>
      <textarea id="contrib" bind:value={howToContribute} required rows="3"
      ></textarea>
    </div>

    <button type="submit" disabled={isLoading}>
      {#if isLoading}
        Submitting...
      {:else}
        Submit Plan
      {/if}
    </button>
  </form>

  {#if statusMessage}
    <p class="status">{statusMessage}</p>
  {/if}
</main>

<style>
  main {
    max-width: 600px;
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
    background-color: #28a745;
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
</style>
