<script lang="ts">
  /**
   * Displays list of all notes found in the user's notes folder.
   */
  import { sessionState, settings, t } from '$lib';
  import type { FormattedNote } from '$lib/types/notes';
  import { listNotes } from '$lib/utils/folder';
  import { readNoteContent } from '$lib/utils/notes';

  let notes: FormattedNote[] = $state([]);
  let isLoading = $state(true);
  let selectedIndex = $state(-1);

  /**
   * Fetches the list of all available notes from the backend.
   */
  const loadNotes = async () => {
    isLoading = true;
    const loadedNotes = await listNotes();
    if (loadedNotes) notes = loadedNotes;
    isLoading = false;
  };

  /**
   * Loads the content of a specific note and sets it as active in the app.
   */
  const selectNote = async (note: FormattedNote) => {
    if (!settings.notesFolder) return;
    const path = `${settings.notesFolder}/${note.filename}`;
    const content = await readNoteContent(path);
    if (content !== null) {
      sessionState.todayNotePath = path;
      sessionState.todayNoteContent = content;
      sessionState.activePopup = null;
    }
  };

  /**
   * Moves the keyboard selection index up or down.
   */
  const moveSelection = (direction: 'up' | 'down') => {
    const count = notes.length;
    if (count === 0) return;
    selectedIndex =
      direction === 'down'
        ? (selectedIndex + 1) % count
        : (selectedIndex - 1 + count) % count;
  };

  /**
   * Handles keyboard events for navigation and actions.
   */
  const handleKeyDown = (e: KeyboardEvent) => {
    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault();
        moveSelection('down');
        break;
      case 'ArrowUp':
        e.preventDefault();
        moveSelection('up');
        break;
      case 'Enter':
        if (selectedIndex >= 0) {
          selectNote(notes[selectedIndex]);
        }
        break;
    }
  };

  $effect(() => {
    if (settings.notesFolder) loadNotes();
  });
</script>

<!-- svelte-ignore a11y_autofocus -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="notes-section" onkeydown={handleKeyDown} tabindex="-1" autofocus>
  {#if isLoading}
    <div class="status-msg">{$t('notes.loading')}</div>
  {:else if notes.length > 0}
    <div class="notes-list" onmouseleave={() => (selectedIndex = -1)}>
      {#each notes as note, i (note.filename)}
        <button
          class="note-item"
          class:selected={i === selectedIndex}
          onclick={() => selectNote(note)}
          onmouseenter={() => (selectedIndex = i)}
        >
          <span class="note-name">{note.formattedName}</span>
        </button>
      {/each}
    </div>
  {:else}
    <div class="status-msg">{$t('notes.list.empty')}</div>
  {/if}
</div>

<style>
  .notes-section {
    display: flex;
    flex-direction: column;
    width: 100%;
    outline: none;
  }

  .notes-list {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    width: 100%;
  }

  .note-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 0.65rem 0.85rem;
    text-align: left;
    background: none;
    border: none;
    color: var(--text-main);
    cursor: pointer;
    font-size: 0.95rem;
    border-radius: 0.5rem;
  }

  .note-item.selected {
    background-color: var(--accent);
    color: var(--accent-text);
  }

  .note-name {
    font-size: 1rem;
    font-weight: 500;
  }

  .status-msg {
    text-align: center;
    padding: 2rem;
    color: var(--text-muted);
    font-style: italic;
    background-color: var(--bg-surface);
    border-radius: 0.5rem;
    border: 1px dashed var(--border);
    font-size: 0.9rem;
  }
</style>
