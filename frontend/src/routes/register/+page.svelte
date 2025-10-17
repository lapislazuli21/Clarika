<script lang="ts">
  import { GraphQLClient } from "graphql-request";

  // --- Configuration ---
  const GQL_ENDPOINT = "http://127.0.0.1:8080/graphql";
  const client = new GraphQLClient(GQL_ENDPOINT);

  interface RegisterUserResponse {
    registerUser: {
      id: string;
      email: string;
    };
  }

  // --- Form State ---
  let email = "";
  let password = "";
  let statusMessage = "";
  let isLoading = false;

  // --- GraphQL Mutation ---
  const REGISTER_MUTATION = `
    mutation RegisterNewUser($email: String!, $password: String!) {
      registerUser(email: $email, password: $password) {
        id
        email
      }
    }
  `;

  // --- Form Submission Logic ---
  async function handleSubmit() {
    isLoading = true;
    statusMessage = "Registering...";

    try {
      const variables = { email, password };
      const response = await client.request<RegisterUserResponse>(
        REGISTER_MUTATION,
        variables
      );

      statusMessage = `Success! User created: ${response.registerUser.email}`;
      // Clear the form on success
      email = "";
      password = "";
    } catch (error) {
      console.error("Registration Error:", error);
      statusMessage = "Registration failed. Please try again.";
    } finally {
      isLoading = false;
    }
  }
</script>

<main>
  <h1>Register New User</h1>

  <form on:submit|preventDefault={handleSubmit}>
    <div class="form-group">
      <label for="email">Email:</label>
      <input type="email" id="email" bind:value={email} required />
    </div>

    <div class="form-group">
      <label for="password">Password:</label>
      <input type="password" id="password" bind:value={password} required />
    </div>

    <button type="submit" disabled={isLoading}>
      {#if isLoading}
        Registering...
      {:else}
        Register
      {/if}
    </button>
  </form>

  {#if statusMessage}
    <p class="status">{statusMessage}</p>
  {/if}
</main>

<style>
  main {
    max-width: 400px;
    margin: 2rem auto;
    padding: 2rem;
    border: 1px solid #ccc;
    border-radius: 8px;
    font-family: sans-serif;
  }
  .form-group {
    margin-bottom: 1rem;
  }
  label {
    display: block;
    margin-bottom: 0.5rem;
  }
  input {
    width: 100%;
    padding: 0.5rem;
    box-sizing: border-box;
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
  button:disabled {
    background-color: #aaa;
  }
  .status {
    margin-top: 1rem;
    text-align: center;
  }
</style>
