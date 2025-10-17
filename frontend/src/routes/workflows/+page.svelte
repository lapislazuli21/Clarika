<script lang="ts">
  import { GraphQLClient } from "graphql-request";
  import { invalidateAll } from "$app/navigation";

  export let data;

  const GQL_ENDPOINT = "http://127.0.0.1:8080/graphql";
  const client = new GraphQLClient(GQL_ENDPOINT);

  let newTemplateName = "";
  let newTemplateDescription = "";
  let isLoading = false;
  let statusMessage = "";

  const CREATE_TEMPLATE_MUTATION = `
    mutation CreateTemplate($name: String!, $description: String) {
      createWorkflowTemplate(name: $name, description: $description) {
        id
      }
    }
  `;

  async function handleCreateTemplate() {
    if (!newTemplateName.trim()) return;
    isLoading = true;
    statusMessage = "";
    try {
      const variables = {
        name: newTemplateName,
        description: newTemplateDescription,
      };
      await client.request(CREATE_TEMPLATE_MUTATION, variables);

      newTemplateName = "";
      newTemplateDescription = "";
      await invalidateAll(); // Refetch the list of templates
    } catch (error) {
      console.error("Error creating template:", error);
      statusMessage = "Failed to create template.";
    } finally {
      isLoading = false;
    }
  }
</script>

<main>
  <h1>Workflow Studio</h1>

  <div class="studio-layout">
    <div class="template-list">
      <h2>Existing Templates</h2>
      {#if data.templates.length > 0}
        <ul>
          {#each data.templates as template (template.id)}
            <a href="/workflows/{template.id}" class="template-link">
              <li>
                <strong>{template.name}</strong>
                <p>{template.description || "No description."}</p>
              </li>
            </a>
          {/each}
        </ul>
      {:else}
        <p>No workflow templates have been created yet.</p>
      {/if}
    </div>

    <div class="create-form">
      <h2>Create New Template</h2>
      <form on:submit|preventDefault={handleCreateTemplate}>
        <div class="form-group">
          <label for="name">Template Name</label>
          <input type="text" id="name" bind:value={newTemplateName} required />
        </div>
        <div class="form-group">
          <label for="description">Description</label>
          <textarea
            id="description"
            bind:value={newTemplateDescription}
            rows="3"
          ></textarea>
        </div>
        <button type="submit" disabled={isLoading}>
          {#if isLoading}Creating...{:else}Create Template{/if}
        </button>
        {#if statusMessage}<p class="error">{statusMessage}</p>{/if}
      </form>
    </div>
  </div>
</main>

<style>
  main {
    max-width: 1200px;
    margin: 2rem auto;
    font-family: sans-serif;
  }
  .studio-layout {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
  }
  ul {
    list-style: none;
    padding: 0;
  }
  li {
    background: #f9f9f9;
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 1rem;
  }
  li p {
    margin: 0.5rem 0 0;
    font-size: 0.9em;
    color: #555;
  }
  .form-group {
    margin-bottom: 1rem;
  }
  label {
    display: block;
    margin-bottom: 0.5rem;
  }
  input,
  textarea {
    width: 100%;
    padding: 0.5rem;
    box-sizing: border-box;
    border: 1px solid #ccc;
    border-radius: 4px;
  }
  button {
    width: 100%;
    padding: 0.75rem;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  .error {
    color: red;
  }
  .template-link {
    text-decoration: none;
    color: inherit;
  }
</style>
