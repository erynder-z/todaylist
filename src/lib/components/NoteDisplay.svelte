<script lang="ts">
  import { untrack } from 'svelte';
  import { NoteLine } from '$lib';
  import type { NoteLineData } from '$lib/types/notes';
  import {
    deleteNoteLine,
    insertNoteLine,
    updateNoteLine,
  } from '$lib/utils/notes';

  let { noteContent, notePath } = $props<{
    noteContent: string;
    notePath: string | null;
  }>();

  let lines = $state<NoteLineData[]>([]);
  let activeIndex = $state<number | null>(null);
  let lastLoadedPath = $state<string | null>(null);
  let changedLineIndex = $state<number | null>(null);

  const flush = async (index: number) => {
    if (lines[index]) {
      const content = lines[index].markdown;
      if (changedLineIndex === index) changedLineIndex = null;
      await updateNoteLine(index, content);
    }
  };

  const loadLines = () => {
    lines = (noteContent || '')
      .split('\n')
      .map((m: string) => ({ markdown: m, html: '' }));
  };

  const ensureTrailingEmptyLine = () => {
    const lastLine = lines[lines.length - 1];
    if (lines.length === 0 || lastLine.markdown.trim() !== '') {
      lines.push({ markdown: '', html: '' });
      insertNoteLine(lines.length - 1, '');
    }
  };

  const syncProps = () => {
    if (notePath !== lastLoadedPath) {
      loadLines();
      ensureTrailingEmptyLine();

      lastLoadedPath = notePath;
      activeIndex = lines.length - 1;
      changedLineIndex = null;
    }
  };

  const autoFlushOnLineSwitch = () => {
    const current = activeIndex;
    untrack(() => {
      if (changedLineIndex !== null && changedLineIndex !== current) {
        flush(changedLineIndex);
      }
    });
  };

  const debouncedAutoSave = () => {
    if (changedLineIndex === null || !lines[changedLineIndex]) return;

    const index = changedLineIndex;
    // Access markdown here so Svelte tracks it and resets the timer on every keystroke
    const _content = lines[index].markdown;

    const timeout = setTimeout(() => {
      untrack(() => {
        if (changedLineIndex === index) flush(index);
      });
    }, 500);

    return () => clearTimeout(timeout);
  };

  const insertLine = async (i: number) => {
    lines.splice(i + 1, 0, { markdown: '', html: '' });
    activeIndex = i + 1;
    await insertNoteLine(i + 1, '');
  };

  const deleteLine = async (i: number) => {
    lines.splice(i, 1);
    activeIndex = Math.max(0, i - 1);
    await deleteNoteLine(i);
  };

  const navigateLines = (i: number, direction: 'up' | 'down') => {
    activeIndex = direction === 'up' ? i - 1 : i + 1;
  };

  const handleKeyDown = async (e: KeyboardEvent, i: number) => {
    switch (e.key) {
      case 'Enter':
        e.preventDefault();
        await insertLine(i);
        break;
      case 'Backspace':
        if (lines[i].markdown === '' && lines.length > 1) {
          e.preventDefault();
          await deleteLine(i);
        }
        break;
      case 'ArrowUp':
        if (i > 0) {
          e.preventDefault();
          navigateLines(i, 'up');
        }
        break;
      case 'ArrowDown':
        if (i < lines.length - 1) {
          e.preventDefault();
          navigateLines(i, 'down');
        }
        break;
    }
  };

  const handleLineChange = (i: number, markdown: string) => {
    if (lines[i]) {
      lines[i].markdown = markdown;
      changedLineIndex = i;
    }
  };

  $effect.pre(() => syncProps());
  $effect(() => autoFlushOnLineSwitch());
  $effect(() => debouncedAutoSave());
</script>

<div class="note-container">
  {#each lines as line, i (i)}
    <NoteLine
      bind:markdown={line.markdown}
      isActive={activeIndex === i}
      onActivate={() => (activeIndex = i)}
      onDeactivate={(e: FocusEvent) => {
        const target = e.relatedTarget as HTMLElement;
        if (!target?.closest('.note-container')) activeIndex = null;
      }}
      onChange={(markdown) => handleLineChange(i, markdown)}
      onKeyDown={(e) => handleKeyDown(e, i)}
    />
  {/each}
</div>

<style>
  .note-container {
    padding: 2rem;
    background-color: var(--bg-surface);
    border: 0.0625rem solid var(--border);
    color: var(--text-main);
    border-radius: 0.75rem;
    margin: 1rem 0;
    max-height: 70vh;
    overflow-y: auto;
  }

  .note-container :global(.rendered-line p) {
    margin: 0;
  }
  .note-container :global(.rendered-line h1) {
    font-size: 1.5rem;
  }
  .note-container :global(.rendered-line h2) {
    font-size: 1.3rem;
  }
  .note-container :global(.rendered-line h3) {
    font-size: 1.2rem;
  }

  .note-container :global(.rendered-line h1),
  .note-container :global(.rendered-line h2),
  .note-container :global(.rendered-line h3) {
    margin-top: 0.5rem;
    margin-bottom: 0.2rem;
    font-weight: 600;
  }

  .note-container :global(.rendered-line ul),
  .note-container :global(.rendered-line ol) {
    margin: 0;
    padding-left: 1.5rem;
  }

  .note-container :global(.rendered-line code) {
    background-color: var(--bg-main);
    padding: 0.2rem 0.4rem;
    border-radius: 0.3rem;
    font-family: monospace;
  }

  .note-container :global(.rendered-line pre) {
    background-color: var(--bg-main);
    padding: 1rem;
    border-radius: 0.5rem;
    overflow-x: auto;
  }

  .note-container :global(.rendered-line blockquote) {
    border-left: 0.25rem solid var(--border);
    margin: 0;
    padding-left: 1rem;
    color: var(--text-muted);
  }

  .note-container :global(.rendered-line a) {
    color: var(--accent);
    text-decoration: none;
  }
  .note-container :global(.rendered-line table) {
    border-collapse: collapse;
    width: 100%;
  }

  .note-container :global(.rendered-line th),
  .note-container :global(.rendered-line td) {
    border: 0.0625rem solid var(--border);
    padding: 0.5rem;
  }
</style>
