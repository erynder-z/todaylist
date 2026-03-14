<script lang="ts">
  /**
   * Component for displaying a note's header, including its date and tags.
   */
  import type { NoteContentResponse } from '$lib/types/notes';
  import NoteDate from './NoteDate.svelte';
  import NoteTags from './NoteTags.svelte';

  let { noteContent } = $props<{
    noteContent: NoteContentResponse | null;
  }>();
</script>

<div class="note-header">
  <NoteDate {noteContent} />
  <NoteTags {noteContent} />
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
</style>
