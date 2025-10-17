<script lang="ts">
  import { GraphQLClient } from "graphql-request";
  import { page } from "$app/stores";
  import { invalidateAll } from "$app/navigation";

  export let data;

  const GQL_ENDPOINT = "http://127.0.0.1:8080/graphql";
  const client = new GraphQLClient(GQL_ENDPOINT);

  // Form state for adding a new step
  let stepName = "";
  let stepOrder = (data.template?.steps.length ?? 0) + 1;
  let role = "";
  let isLoading = false;

  const ADD_STEP_MUTATION = `
    mutation AddStep($templateId: ID!, $stepName: String!, $stepOrder: Int!, $role: String) {
      addWorkflowStep(templateId: $templateId, stepName: $stepName, stepOrder: $stepOrder, role: $role) {
        id
      }
    }
  `;

  async function handleAddStep() {
    isLoading = true;
    try {
      const variables = {
        templateId: $page.params.id,
        stepName,
        stepOrder,
        role: role || null, // Send null if the string is empty
      };
      await client.request(ADD_STEP_MUTATION, variables);
      await invalidateAll(); // Refetch data to show the new step

      // Reset form for the next step
      stepName = "";
      role = "";
      stepOrder++; // Automatically increment for the next step
    } catch (error) {
      console.error("Error adding step:", error);
      alert("Failed to add step.");
    } finally {
      isLoading = false;
    }
  }
</script>

<main>
  {#if data.template}
    <h1>{data.template.name}</h1>
    <p>{data.template.description}</p>

    <div class="layout">
      <div class="step-list">
        <h2>Steps</h2>
        {#if data.template.steps.length > 0}
          <ol>
            {#each data.template.steps as step (step.id)}
              <li>
                <strong>{step.stepName}</strong> (# {step.stepOrder})<br />
                {#if step.role}<small>Role: {step.role}</small>{/if}
              </li>
            {/each}
          </ol>
        {:else}
          <p>No steps have been added to this template yet.</p>
        {/if}
      </div>

      <div class="add-step-form">
        <h2>Add New Step</h2>
        <form on:submit|preventDefault={handleAddStep}>
          <div class="form-group">
            <label for="stepOrder">Order</label>
            <input
              type="number"
              id="stepOrder"
              bind:value={stepOrder}
              required
              min="1"
            />
          </div>
          <div class="form-group">
            <label for="stepName">Step Name</label>
            <input type="text" id="stepName" bind:value={stepName} required />
          </div>
          <div class="form-group">
            <label for="role">Placeholder Role (Optional)</label>
            <input
              type="text"
              id="role"
              bind:value={role}
              placeholder="e.g., Lead Developer"
            />
          </div>
          <button type="submit" disabled={isLoading}>Add Step</button>
        </form>
      </div>
    </div>
  {:else}
    <p>{data.error || "Template not found."}</p>
  {/if}
</main>

<style>
  main {
    max-width: 1000px;
    margin: 2rem auto;
  }
  .layout {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
  }
  ol {
    padding-left: 20px;
  }
  li {
    margin-bottom: 1rem;
  }
  .form-group {
    margin-bottom: 1rem;
  }
  label {
    display: block;
  }
  input {
    width: 100%;
    padding: 0.5rem;
    box-sizing: border-box;
  }
  button {
    width: 100%;
    padding: 0.75rem;
  }
</style>
