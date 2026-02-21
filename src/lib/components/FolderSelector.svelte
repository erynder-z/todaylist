<script lang="ts">
  import {
    checkTodaysNoteExists,
    createTodaysNote,
    config,
    selectFolder,
  } from '$lib';

  const handleSelectFolder = async () => {
    const selectedPath = await selectFolder();

    if (selectedPath) {
      const currentConfig = $config;
      await config.save({ ...currentConfig, notes_folder: selectedPath });

      const exists = await checkTodaysNoteExists();
      if (!exists) await createTodaysNote();
    }
  };
</script>

<div class="folder-section">
  <button onclick={handleSelectFolder} class="folder-button">
    {#if $config.notes_folder}
      Change Folder
    {:else}
      Select Folder
    {/if}
  </button>

  {#if $config.notes_folder}
    <p class="folder-path">Selected folder: {$config.notes_folder}</p>
  {/if}
</div>

<style>
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
    transition:
      background-color 0.2s,
      transform 0.1s;
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
