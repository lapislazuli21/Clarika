<script lang="ts">
  export let data;
</script>

<main>
  <h1>My Projects</h1>

  {#if data.error}
    <p class="error">{data.error}</p>
  {:else if data.projects.length === 0}
    <p>You have not created any projects yet.</p>
  {:else}
    <div class="project-list">
      {#each data.projects as project (project.id)}
        <a href="/projects/{project.id}" class="project-link">
          <div class="project-card">
            <h2>{project.name}</h2>
            <p>{project.description || "No description provided."}</p>
            {#if project.deadline}
              <p class="deadline">
                <strong>Deadline:</strong>
                {new Date(project.deadline).toLocaleDateString()}
              </p>
            {/if}
          </div>
        </a>
      {/each}
    </div>
  {/if}
</main>

<style>
  main {
    max-width: 900px;
    margin: 2rem auto;
    font-family: sans-serif;
  }
  .error {
    color: red;
  }
  .project-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 1.5rem;
  }
  .project-card {
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 1.5rem;
    background-color: #f9f9f9;
  }
  .project-card h2 {
    margin-top: 0;
  }
  .deadline {
    font-size: 0.9em;
    color: #555;
  }
  .project-link {
    text-decoration: none;
    color: inherit;
  }
  .project-card {
    /* Add a transition for a nice hover effect */
    transition: transform 0.2s ease-in-out;
  }
  .project-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }
</style>
