<script lang="ts">
  /**
   * Component for displaying a note's formatted date.
   * Extracts the localized date from the provided note content.
   */
  import type { NoteContentResponse } from '$lib/types/notes';

  let { noteContent } = $props<{
    noteContent: NoteContentResponse | null;
  }>();

  let date = $derived(noteContent?.metadata.formattedDate);
</script>

{#if date}
  <span class="date">{date}</span>
{/if}

<style>
  .date {
    font-size: 0.875rem;
    color: var(--accent);
    font-weight: 500;
  }
</style>
