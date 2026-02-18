<script lang="ts">
import { config, createNewNote, selectFolder } from "$lib";

let currentDate = $state(new Date().toLocaleDateString());

async function handleSelectFolder() {
	const selectedPath = await selectFolder();

	if (selectedPath) {
		const currentConfig = $config;
		await config.save({ ...currentConfig, notes_folder: selectedPath });
	}
}

async function handleCreateNote() {
	if (!$config.notes_folder) {
		alert("Please select a folder first");
		return;
	}

	const filePath = await createNewNote();
	if (filePath) {
		alert(`Created new note: ${filePath}`);
	}
}
</script>

<main class="container">
  <h1>Today is: {currentDate}</h1>  
                  
  <div class="folder-section">        
    <button onclick={handleSelectFolder} class="folder-button">
      Select Folder       
    </button> 
    
    <button onclick={handleCreateNote} class="folder-button" style="margin-top: 1rem;">
      Create New Note
    </button>
      
    {#if $config.notes_folder}
      <p class="folder-path">Selected folder: {$config.notes_folder}</p>
    {/if}
  </div>
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
  }

  .folder-section {
    margin-top: 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .folder-button {
    padding: 0.8rem 1.5rem;
    background-color: #646cff;
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s, transform 0.1s;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .folder-button:hover {
    background-color: #535bf2;
  }

  .folder-button:active {
    transform: scale(0.98);
  }

  .folder-path {
    max-width: 80%;
    word-break: break-all;
    text-align: center;
    padding: 0.5rem 1rem;
    background-color: rgba(0, 0, 0, 0.05);
    border-radius: 6px;
    border: 1px solid rgba(0, 0, 0, 0.1);
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    .folder-button {
      background-color: #747bff;
    }

    .folder-button:hover {
      background-color: #646cff;
    }

    .folder-path {
      background-color: rgba(255, 255, 255, 0.1);
      border: 1px solid rgba(255, 255, 255, 0.2);
    }
  }
</style>