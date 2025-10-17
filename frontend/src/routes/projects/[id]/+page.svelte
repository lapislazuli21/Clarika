<script lang="ts">
  export let data;

  import { GraphQLClient } from "graphql-request";
  import { page } from "$app/stores";
  import { invalidateAll } from "$app/navigation";

  const GQL_ENDPOINT = "http://127.0.0.1:8080/graphql";
  const client = new GraphQLClient(GQL_ENDPOINT);

  let newTaskTitle = "";
  let isLoading = false;
  let statusMessage = "";

  let selectedTemplateId = "";

  const statuses = [
    "NotStarted",
    "InProgress",
    "Blocked",
    "UnderReview",
    "Deprecated",
    "Completed",
  ];

  const raciRoles = ["Responsible", "Accountable", "Consulted", "Informed"];

  const CREATE_TASK_MUTATION = `
    mutation CreateNewTask($projectId: ID!, $title: String!) {
      createTask(projectId: $projectId, title: $title) {
        id
      }
    }
  `;

  const UPDATE_TASK_STATUS_MUTATION = `
    mutation UpdateTaskStatus($taskId: ID!, $status: TaskStatus!) {
      updateTaskStatus(taskId: $taskId, status: $status) {
        id
        status
      }
    }
  `;

  const ASSIGN_RACI_ROLE_MUTATION = `
    mutation AssignRole($userId: ID!, $taskId: ID!, $role: RaciRole!) {
      assignRaciRole(userId: $userId, taskId: $taskId, role: $role) {
        role
        user { id }
      }
    }
  `;

  const APPLY_TEMPLATE_MUTATION = `
    mutation ApplyTemplate($templateId: ID!, $projectId: ID!) {
      applyWorkflowTemplateToProject(templateId: $templateId, projectId: $projectId) {
        id
      }
    }
  `;

  const LINK_JIRA_TICKET_MUTATION = `
    mutation LinkTicket($taskId: ID!, $jiraId: String!) {
      linkJiraTicket(taskId: $taskId, jiraTicketId: $jiraId) {
        id
        jiraTicketId
      }
    }
  `;

  async function handleAddTask() {
    if (!newTaskTitle.trim()) return; // Don't submit empty tasks
    isLoading = true;
    statusMessage = "";

    try {
      const variables = {
        // Get the project ID from the page's URL parameters
        projectId: $page.params.id,
        title: newTaskTitle,
      };
      await client.request(CREATE_TASK_MUTATION, variables);

      // Clear the input and refetch all data for the page
      newTaskTitle = "";
      await invalidateAll();
    } catch (error) {
      console.error("Error creating task:", error);
      statusMessage = "Failed to create task.";
    } finally {
      isLoading = false;
    }
  }

  async function handleStatusChange(taskId: string, newStatus: string) {
    if (!data.project) return;
    try {
      const variables = { taskId, status: newStatus };
      await client.request(UPDATE_TASK_STATUS_MUTATION, variables);

      // For a better user experience, updating the local data instantly instead of refetching the whole page.
      const taskIndex = data.project.tasks.findIndex((t) => t.id === taskId);
      if (taskIndex !== -1) {
        data.project.tasks[taskIndex].status = newStatus;
      }
    } catch (error) {
      console.error("Error updating task status:", error);
      alert("Failed to update task status.");
    }
  }

  async function handleAssignRole(taskId: string, event: Event) {
    const form = event.target as HTMLFormElement;
    const formData = new FormData(form);
    const userId = formData.get("user") as string;
    const role = formData.get("role") as string;

    if (!userId || !role) return;

    try {
      const variables = { taskId, userId, role };
      await client.request(ASSIGN_RACI_ROLE_MUTATION, variables);
      await invalidateAll(); // Refetch all data to show the new assignment
      form.reset(); // Clear the form after submission
    } catch (error) {
      console.error("Error assigning RACI role:", error);
      alert("Failed to assign role.");
    }
  }

  async function handleApplyTemplate() {
    if (!selectedTemplateId) {
      alert("Please select a template to apply.");
      return;
    }
    try {
      const variables = {
        templateId: selectedTemplateId,
        projectId: $page.params.id,
      };
      await client.request(APPLY_TEMPLATE_MUTATION, variables);
      await invalidateAll(); // Magically refetches tasks
    } catch (error) {
      console.error("Error applying template:", error);
      alert("Failed to apply workflow template.");
    }
  }

  async function handleLinkJiraTicket(taskId: string, event: Event) {
    const form = event.target as HTMLFormElement;
    const formData = new FormData(form);
    const jiraId = formData.get("jiraId") as string;

    if (!jiraId) return;

    try {
      const variables = { taskId, jiraId };
      await client.request(LINK_JIRA_TICKET_MUTATION, variables);
      await invalidateAll(); // Refetch data to show the new link
    } catch (error) {
      console.error("Error linking Jira ticket:", error);
      alert("Failed to link Jira ticket.");
    }
  }
</script>

<main>
  {#if data.project}
    <div class="project-header">
      <h1>{data.project.name}</h1>
      {#if data.project.deadline}
        <p class="deadline">
          <strong>Deadline:</strong>
          {new Date(data.project.deadline).toLocaleDateString()}
        </p>
      {/if}
      <p class="description">{data.project.description || "No description."}</p>
    </div>

    <div class="workflow-apply-section">
      <h3>Apply Workflow Template</h3>
      <div class="apply-form">
        <select bind:value={selectedTemplateId}>
          <option value="">-- Select an SOP --</option>
          {#each data.templates as template (template.id)}
            <option value={template.id}>{template.name}</option>
          {/each}
        </select>
        <button on:click={handleApplyTemplate}>Apply</button>
      </div>
    </div>

    <div class="task-section">
      <h2>Tasks</h2>
      <form class="add-task-form" on:submit|preventDefault={handleAddTask}>
        <input
          type="text"
          bind:value={newTaskTitle}
          placeholder="Add a new task..."
          required
        />
        <button type="submit" disabled={isLoading}>
          {#if isLoading}Adding...{:else}Add Task{/if}
        </button>
      </form>
      <div class="task-grid">
        {#if data.project.tasks.length === 0}
          <p>No tasks have been added to this project yet.</p>
        {:else}
          {#each data.project.tasks as task (task.id)}
            <div class="task-card">
              <div class="task-header">
                <h3>{task.title}</h3>
                <div class="task-meta">
                  <label for="status-{task.id}">Status:</label>
                  <select
                    id="status-{task.id}"
                    bind:value={task.status}
                    on:change={() => handleStatusChange(task.id, task.status)}
                  >
                    {#each statuses as statusValue}
                      <option value={statusValue}
                        >{statusValue.replace(/([A-Z])/g, " $1").trim()}</option
                      >
                    {/each}
                  </select>
                </div>
              </div>

              <div class="jira-section">
                {#if task.jiraTicketId}
                  <p>
                    <strong>Jira Link:</strong>
                    <a
                      href={`https://gravityintel.atlassian.net/browse/${task.jiraTicketId}`}
                      target="_blank">{task.jiraTicketId}</a
                    >
                  </p>
                {:else}
                  <form
                    class="jira-form"
                    on:submit|preventDefault={(e) =>
                      handleLinkJiraTicket(task.id, e)}
                  >
                    <input
                      type="text"
                      name="jiraId"
                      placeholder="PROJ-123"
                      required
                    />
                    <button type="submit">Link Jira Ticket</button>
                  </form>
                {/if}
              </div>

              <div class="assignments">
                <h4>Assignments:</h4>
                {#if task.raciAssignments && task.raciAssignments.length > 0}
                  <ul>
                    {#each task.raciAssignments as assignment}
                      <li>
                        <strong>{assignment.role}:</strong>
                        {assignment.user.email}
                      </li>
                    {/each}
                  </ul>
                {:else}
                  <p>No roles assigned yet.</p>
                {/if}
              </div>

              <form
                class="raci-form"
                on:submit|preventDefault={(e) => handleAssignRole(task.id, e)}
              >
                <select name="user" required>
                  <option value="">-- Select User --</option>
                  {#each data.users as user (user.id)}
                    <option value={user.id}>{user.email}</option>
                  {/each}
                </select>
                <select name="role" required>
                  <option value="">-- Select Role --</option>
                  {#each raciRoles as role}
                    <option value={role}>{role}</option>
                  {/each}
                </select>
                <button type="submit">Assign</button>
              </form>
            </div>
          {/each}
        {/if}
      </div>
    </div>
  {:else if data.error}
    <p class="error">{data.error}</p>
  {:else}
    <p>Project not found.</p>
  {/if}
</main>

<style>
  main {
    max-width: 800px;
    margin: 2rem auto;
    font-family: sans-serif;
  }
  .project-header {
    padding-bottom: 1.5rem;
    border-bottom: 1px solid #eee;
  }
  h1 {
    margin-bottom: 0.5rem;
  }
  .deadline {
    color: #555;
    margin-top: 0;
  }
  .task-section {
    margin-top: 2rem;
  }
  .task-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: 1.5rem;
  }
  .task-card {
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
  }
  .task-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1rem;
  }
  .task-header h3 {
    margin: 0;
  }
  .task-meta {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9em;
  }
  .task-meta select {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    border: 1px solid #ccc;
  }
  .assignments {
    flex-grow: 1;
  }
  .assignments ul {
    padding-left: 20px;
    margin: 0.5rem 0;
  }
  .assignments p {
    font-style: italic;
    color: #666;
  }
  .raci-form {
    display: flex;
    gap: 0.5rem;
    margin-top: 1rem;
  }
  .raci-form select {
    flex-grow: 1;
    padding: 0.25rem;
  }
  .raci-form button {
    background-color: #28a745;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    padding: 0.5rem 1rem;
  }
  .error {
    color: red;
  }
  .add-task-form {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
  }
  .add-task-form input {
    flex-grow: 1;
    padding: 0.5rem;
    border: 1px solid #ccc;
    border-radius: 4px;
  }
  .add-task-form button {
    padding: 0.5rem 1rem;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  .add-task-form button:disabled {
    background-color: #aaa;
  }
  .workflow-apply-section {
    background: #f0f4f8;
    padding: 1.5rem;
    border-radius: 8px;
    margin-bottom: 2rem;
  }
  .workflow-apply-section h3 {
    margin-top: 0;
  }
  .apply-form {
    display: flex;
    gap: 1rem;
  }
  .apply-form select {
    flex-grow: 1;
    padding: 0.5rem;
  }
  .apply-form button {
    background-color: #6c757d;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 0.5rem 1.5rem;
    cursor: pointer;
  }
  .jira-section {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid #eee;
  }
  .jira-section p {
    margin: 0;
    font-size: 0.9em;
  }
  .jira-form {
    display: flex;
    gap: 0.5rem;
  }
  .jira-form input {
    flex-grow: 1;
    padding: 0.25rem;
  }
  .jira-form button {
    background-color: #0052cc; /* Atlassian Blue */
    font-size: 0.8em;
    padding: 0.25rem 0.75rem;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
</style>
