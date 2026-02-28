<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { settings, t } from '$lib';
  import type { FolderValidation } from '$lib/types/folder';
  import { selectFolder } from '$lib/utils/folder';

  let selectedFolderPath = $state<string | null>(null);
  let validationResult = $state<FolderValidation | null>(null);

  let isUseFolderButtonEnabled = $derived(
    selectedFolderPath !== null &&
      validationResult !== null &&
      validationResult.is_valid &&
      validationResult.is_writable &&
      selectedFolderPath !== $settings.notes_folder,
  );

  const handleSelectFolder = async () => {
    const path = await selectFolder();

    if (path) {
      selectedFolderPath = path;
      try {
        const result: FolderValidation = await invoke('validate_folder', {
          path,
        });
        validationResult = result;
      } catch (error) {
        console.error('Validation failed:', error);
        validationResult = {
          is_valid: false,
          is_writable: false,
          exists: false,
          note_count: 0,
          error: String(error),
        };
      }
    }
  };

  const handleUseFolder = async () => {
    if (selectedFolderPath && isUseFolderButtonEnabled) {
      await settings.switchNotesFolder(selectedFolderPath);
      selectedFolderPath = null;
      validationResult = null;
    }
  };
</script>

<div class="setting-item">
  <label for="folder-select">{$t('settings.folder.title')}</label>
  <div class="button-container">
    <button onclick={handleSelectFolder} class="folder-button">
      {$t('settings.folder.select')}
    </button>

    <button
      onclick={handleUseFolder}
      class="folder-button use-folder-button"
      disabled={!isUseFolderButtonEnabled}
    >
      {$t('settings.save')}
    </button>
  </div>

  <div class="folder-status">
    {#if selectedFolderPath}
      <p class="folder-path">
        <strong>{$t('settings.folder.selected')}</strong>
        {selectedFolderPath}
      </p>
      {#if validationResult}
        <p
          class="validation-msg"
          class:error={!validationResult.is_valid ||
            !validationResult.is_writable}
        >
          {#if !validationResult.is_valid || !validationResult.is_writable}
            {$t('settings.folder.validation.invalid', {
              error: validationResult.error || 'Unknown error',
            })}
          {:else if validationResult.exists}
            {$t('settings.folder.validation.valid_existing', {
              count: validationResult.note_count,
            })}
          {:else}
            {$t('settings.folder.validation.valid_new')}
          {/if}
        </p>
      {/if}
    {:else if $settings.notes_folder}
      <p class="folder-path">
        <strong>{$t('settings.folder.current')}</strong>
        {$settings.notes_folder}
      </p>
    {:else}
      <p class="folder-path">{$t('settings.folder.no_folder')}</p>
    {/if}
  </div>
</div>

<style>
  .setting-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.8rem;
    width: 100%;
  }

  label {
    font-weight: 600;
    font-size: 1.1rem;
  }

  .button-container {
    display: flex;
    gap: 0.5rem;
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

  .folder-button:hover:not(:disabled) {
    background-color: #535bf2;
  }

  .folder-button:active:not(:disabled) {
    transform: scale(0.98);
  }

  .folder-button:disabled {
    background-color: #cccccc;
    cursor: not-allowed;
    box-shadow: none;
  }

  .folder-status {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
  }

  .folder-path {
    max-width: 90%;
    word-break: break-all;
    text-align: center;
    padding: 0.5rem 1rem;
    background-color: rgba(0, 0, 0, 0.05);
    border-radius: 6px;
    border: 1px solid rgba(0, 0, 0, 0.1);
    font-size: 0.9rem;
    margin: 0;
  }

  .validation-msg {
    font-size: 0.85rem;
    color: #28a745;
    margin: 0;
  }

  .validation-msg.error {
    color: #dc3545;
  }

  .use-folder-button {
    background-color: #28a745;
  }

  .use-folder-button:hover:not(:disabled) {
    background-color: #218838;
  }

  @media (prefers-color-scheme: dark) {
    .folder-button {
      background-color: #747bff;
    }

    .folder-path {
      background-color: rgba(255, 255, 255, 0.1);
      border: 1px solid rgba(255, 255, 255, 0.2);
    }

    .validation-msg {
      color: #4cd137;
    }

    .validation-msg.error {
      color: #ff7675;
    }
  }
</style>
