<script lang="ts">
  /**
   * Component for displaying note sections with shortcut hints for quick navigation.
   */

  import type { NoteSection } from '$lib/types/notes';
  import { tagSuggestionShortcuts } from '../config/shortcuts';
  import { inputManager } from '../stores/input.svelte';

  let { sections, onSelect } = $props<{
    sections: NoteSection[];
    onSelect: (name: string) => void;
  }>();

  // Show only up to 20 sections as there are only 20 shortcuts (1-9, A-K)
  let visibleSections = $derived(sections.slice(0, 20));
</script>

{#if visibleSections.length > 0}
  <div class="sections-container">
    {#each visibleSections as section, i}
      <button class="section-pill" onclick={() => onSelect(section.name)}>
        <span class="section-name">{section.name}</span>
        <span class="shortcut-hint">
          <span class="mod">{inputManager.primaryLabel}</span>
          <span class="mod">{inputManager.secondaryLabel}</span>
          <span class="key">{tagSuggestionShortcuts.labels[i]}</span>
        </span>
      </button>
    {/each}
  </div>
{/if}

<style>
  .sections-container {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    padding: 0.5rem;
    background-color: color-mix(in srgb, var(--accent), transparent 95%);
    border-radius: 0.5rem;
    border: 1px solid var(--border);
  }

  .section-pill {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.25rem 0.5rem;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 0.375rem;
    color: var(--text-main);
    cursor: pointer;
    font-size: 0.85rem;
    transition: all 0.15s ease;
  }

  .section-pill:hover {
    background-color: var(--accent);
    color: var(--accent-text);
    border-color: var(--accent);
  }

  .section-name {
    font-weight: 500;
  }

  .shortcut-hint {
    font-family: var(--font-mono, monospace);
    font-size: 0.7rem;
    color: var(--text-muted);
    border: 1px solid var(--border);
    padding: 0.1rem 0.25rem;
    border-radius: 0.25rem;
    display: flex;
    align-items: center;
    gap: 0.15rem;
    opacity: 0.8;
  }

  .section-pill:hover .shortcut-hint {
    color: var(--accent-text);
    border-color: var(--accent-text);
    opacity: 1;
  }

  .mod {
    font-size: 0.6rem;
    opacity: 0.7;
  }

  .key {
    font-weight: 600;
  }
</style>
