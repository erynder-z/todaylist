<script lang="ts">
import { onMount } from "svelte";
import { config, FolderSelector, NotesList } from "$lib";

let currentDate = $state(new Date().toLocaleDateString());

onMount(async () => {
	await config.load();
});
</script>

<main class="container">
  <h1>Today is: {currentDate}</h1>  
  
  <FolderSelector />
  <NotesList />
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  h1 {
    text-align: center;
    color: #333;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    h1 {
      color: #f6f6f6;
    }
  }
</style>