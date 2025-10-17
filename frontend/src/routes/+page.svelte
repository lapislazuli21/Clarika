<script lang="ts">
  export let data;
</script>

<main>
  <h1>Clarity Dashboard</h1>
  <p>Welcome to the main dashboard.</p>

  {#if data.status === "ok"}
    <p class="status-ok"><strong>Backend Status:</strong> {data.message}</p>
  {:else}
    <p class="status-error"><strong>Backend Status:</strong> {data.message}</p>
  {/if}

  <nav>
    <a href="/register">Go to Registration Page</a>
    <a href="/growth_plan">Fill Out My Growth Plan</a>
    <a href="/ai_scoper">AI Project Scoper</a>
    <a href="/projects">View Projects</a>
    <a href="/workflows">Workflow Studio</a>
  </nav>
</main>

<style>
  main {
    font-family: sans-serif;
    padding: 2em;
    text-align: center;
  }
  .status-ok {
    color: #28a745; /* Green */
  }
  .status-error {
    color: #dc3545; /* Red */
  }
  nav {
    margin-top: 2em;
    display: flex;
    flex-direction: column;
    gap: 1em;
  }
</style>
