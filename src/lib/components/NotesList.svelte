<script lang="ts">
  import { onMount } from 'svelte';
  import { config } from '$lib';
  import type { FormattedNote } from '$lib/types/notes';
  import { listNotes } from '$lib/utils/folder';

  let notes: FormattedNote[] = $state([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);

  onMount(() => {
    const unsubscribe = config.subscribe(($config) => {
      if ($config.notes_folder) {
        loadNotes();
      } else {
        notes = [];
        isLoading = false;
      }
    });

    return unsubscribe;
  });

  const loadNotes = async () => {
    try {
      isLoading = true;
      error = null;

      if (!$config.notes_folder) {
        isLoading = false;
        return;
      }

      const loadedNotes = await listNotes();
      if (loadedNotes) notes = loadedNotes;
    } catch (err) {
      console.error('Error loading notes:', err);
      error = 'Failed to load notes';
    } finally {
      isLoading = false;
    }
  };
</script>

<div class="notes-section">
  {#if isLoading && $config.notes_folder}
    <div class="loading">Loading notes...</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else if $config.notes_folder && notes.length > 0}
    <h2>Your Notes</h2>
    <div class="notes-list">
      {#each notes as note (note.filename)}
        <div class="note-item">
          <span class="note-name">{note.formatted_name}</span>
        </div>
      {/each}
    </div>
  {:else if $config.notes_folder}
    <div class="empty-notes">No notes found. Create your first note!</div>
  {/if}
</div>

<style>
  h2 {
    margin: 2rem 0 1rem;
    font-size: 1.5rem;
    color: #444;
    text-align: center;
  }

  .notes-section {
    margin: 2rem auto;
    max-width: 800px;
    width: 100%;
    padding: 0 1rem;
  }

  .notes-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    max-width: 600px;
    margin: 0 auto;
    width: 100%;
  }

  .note-item {
    padding: 1rem;
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    transition:
      transform 0.2s,
      box-shadow 0.2s;
    cursor: pointer;
    text-align: left;
  }

  .note-item:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
  }

  .note-name {
    font-size: 1rem;
    font-weight: 500;
    color: #333;
  }

  .loading,
  .error,
  .empty-notes {
    padding: 1rem;
    border-radius: 8px;
    margin: 2rem auto 0;
    max-width: 500px;
    text-align: center;
  }

  .loading {
    color: #666;
  }

  .error {
    background-color: #ffebee;
    color: #c62828;
    border: 1px solid #ef9a9a;
  }

  .empty-notes {
    color: #999;
  }

  @media (prefers-color-scheme: dark) {
    h2 {
      color: #f6f6f6;
    }

    .note-item {
      background-color: #3a3a3a;
      color: #f6f6f6;
    }

    .note-name {
      color: #f6f6f6;
    }

    .error {
      background-color: #3a2a2a;
      color: #ffb7b7;
      border: 1px solid #5a3a3a;
    }
  }
</style>
