<script lang="ts">
  import type { NoteContentResponse } from '$lib/types/notes';

  let { noteContent } = $props<{
    noteContent: NoteContentResponse | null;
  }>();

  let formattedDate = $derived(noteContent?.metadata.formattedDate);
  let tags = $derived(noteContent?.metadata.tags || []);
</script>

<div class="note-header">
  {#if formattedDate}
    <span class="date">{formattedDate}</span>
  {/if}
  {#if tags.length > 0}
    <div class="tags-container">
      {#each tags as tag}
        <span class="tag-pill">{tag}</span>
      {/each}
    </div>
  {/if}
</div>

<style>
  .note-header {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1.5rem;
    padding-bottom: 0.75rem;
    border-bottom: 0.0625rem solid var(--border);
  }

  .date {
    font-size: 0.875rem;
    color: var(--accent);
    font-weight: 500;
  }

  .tags-container {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
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
</style>
