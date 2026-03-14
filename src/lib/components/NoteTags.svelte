<script lang="ts">
  /**
   * Component for displaying and managing tags for a note.
   * Allows users to view existing tags and add new ones.
   */
  import { sessionState, t } from '$lib';
  import type { NoteContentResponse } from '$lib/types/notes';
  import { addNoteTag } from '$lib/utils/notes';

  let { noteContent } = $props<{
    noteContent: NoteContentResponse | null;
  }>();

  let tags = $derived(noteContent?.metadata.tags || []);
  let isAddingTag = $state(false);
  let newTag = $state('');

  /**
   * Triggers the addition of a new tag and updates the session state.
   */
  const handleAddTag = async () => {
    if (newTag.trim()) {
      const updatedContent = await addNoteTag(newTag.trim());
      if (updatedContent) sessionState.todayNoteContent = updatedContent;
    }
    newTag = '';
    isAddingTag = false;
  };

  /**
   * Handles keyboard interactions for the tag input field.
   */
  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleAddTag();
    } else if (e.key === 'Escape') {
      isAddingTag = false;
      newTag = '';
    }
  };
</script>

<div class="tags-container">
  {#each tags as tag}
    <span class="tag-pill">{tag}</span>
  {/each}

  {#if isAddingTag}
    <!-- svelte-ignore a11y_autofocus -->
    <input
      type="text"
      class="tag-input"
      bind:value={newTag}
      onkeydown={handleKeyDown}
      onblur={() => (isAddingTag = false)}
      autofocus
      placeholder={$t('tag.placeholder')}
    />
  {:else}
    <button
      class="add-tag-btn"
      onclick={() => (isAddingTag = true)}
      aria-label="Add tag"
    >
      +
    </button>
  {/if}
</div>

<style>
  .tags-container {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: center;
  }

  .tag-pill {
    font-size: 0.75rem;
    background-color: var(--accent);
    color: var(--accent-text);
    padding-inline: 0.625rem;
    padding-block: 0.125rem;
    border-radius: 999rem;
    font-weight: 600;
    white-space: nowrap;
  }

  .add-tag-btn {
    font-size: 0.75rem;
    background-color: var(--bg-main);
    color: var(--text-muted);
    border: 0.0625rem dashed var(--border);
    padding-inline: 0.5rem;
    padding-block: 0.125rem;
    border-radius: 999rem;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .add-tag-btn:hover {
    border-color: var(--accent);
    color: var(--accent);
  }

  .tag-input {
    font-size: 0.75rem;
    background-color: var(--bg-main);
    color: var(--text-main);
    border: 0.0625rem solid var(--accent);
    padding-inline: 0.625rem;
    padding-block: 0.125rem;
    border-radius: 999rem;
    outline: none;
    width: 4rem;
  }
</style>
