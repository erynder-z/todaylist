<script lang="ts">
  /**
   * Slide-in menu for displaying thread options
   */
  import { slide } from 'svelte/transition';
  import type { NoteThread } from '$lib/interfaces/notes';
  import { toast } from '$lib/stores/toast.svelte';
  import { t } from '$lib/utils/i18n';
  import { stripMarkdown } from '$lib/utils/markdown';
  import { sessionState } from '../stores/sessionState.svelte';
  import { notesService } from '../utils/notes';
  import { useShortcuts } from '../utils/shortcuts';
  import KeyboardShortcut from './KeyboardShortcut.svelte';

  let { thread } = $props<{
    thread: NoteThread;
  }>();

  let aggregatedThread = $state<Awaited<
    ReturnType<typeof notesService.aggregateThread>
  > | null>(null);

  let hasLinkedThreads = $state(false);

  /**
   * Whether the thread has any non-empty content lines (beyond the header)
   */
  let hasContent = $derived.by(() => {
    const content = sessionState.todayNoteContent?.content;
    if (!content) return false;

    const lines = content.split('\n');
    // Content lines are between startLine + 1 (skip !!! header) and endLine
    const contentLines = lines.slice(thread.startLine + 1, thread.endLine);
    return contentLines.some((line) => line.trim().length > 0);
  });

  /**
   * Closes the menu
   */
  const closeMenu = () => {
    sessionState.activePopup = null;
  };

  /**
   * Loads linked thread information
   */
  const loadLinkedThreads = async () => {
    try {
      aggregatedThread = await notesService.aggregateThread(thread.name);
      hasLinkedThreads =
        aggregatedThread !== null && aggregatedThread.items.length > 1;
    } catch {
      aggregatedThread = null;
      hasLinkedThreads = false;
    }
  };

  /**
   * Opens linked threads popup
   */
  const handleLinked = () => {
    if (!aggregatedThread || !hasLinkedThreads) {
      return;
    }

    sessionState.aggregatedThread = aggregatedThread;
    sessionState.activePopup = 'threadAggregation';
  };

  /**
   * Copies the thread content to the clipboard after stripping the markdown from it
   */
  const handleCopyThread = async () => {
    try {
      const content = sessionState.todayNoteContent?.content;

      if (!content || !hasContent) return;

      const lines = content.split('\n');

      const threadContent = lines
        .slice(thread.startLine + 1, thread.endLine)
        .join('\n');

      const plainText = stripMarkdown(threadContent);

      await navigator.clipboard.writeText(plainText);

      toast.success($t('thread.options.copy_success'));

      closeMenu();
    } catch {
      toast.error($t('thread.options.copy_error'));
    }
  };

  /**
   * Toggles the pin status of the current thread
   */
  const handleTogglePin = async () => {
    try {
      const currentContent = sessionState.todayNoteContent?.content;

      if (!currentContent) {
        toast.error($t('thread.options.pin_error'));
        return;
      }

      const result = await notesService.toggleThreadPin(
        thread.id,
        currentContent,
      );

      // Update the session state with the new content
      if (result) {
        // Force a refresh by creating a new object with the same data
        const newContent = JSON.parse(JSON.stringify(result));
        sessionState.todayNoteContent = newContent;
        const updatedThread = result.threads.find((t) => t.id === thread.id);
        if (updatedThread)
          sessionState.selectedThreadForOptions = updatedThread;
      }
    } catch {
      toast.error($t('thread.options.pin_error'));
    }
  };

  /**
   * Deletes the current thread
   */
  const handleRemoveThread = async () => {
    try {
      const currentContent = sessionState.todayNoteContent?.content;

      if (!currentContent) {
        toast.error($t('thread.options.remove_error'));
        return;
      }

      const result = await notesService.removeThread(thread.id, currentContent);

      // Update the session state with the new content
      if (result) {
        // Force a refresh by creating a new object with the same data
        const newContent = JSON.parse(JSON.stringify(result));
        sessionState.todayNoteContent = newContent;
      }

      closeMenu();
    } catch (error) {
      toast.error($t('thread.options.remove_error'));
    }
  };

  $effect(() => {
    thread;
    loadLinkedThreads();
  });

  useShortcuts('thread-options', {
    threadOptionRemove: handleRemoveThread,
    threadOptionLinked: handleLinked,
    threadOptionCopy: handleCopyThread,
    threadOptionPin: handleTogglePin,
    closePopup: closeMenu,
  });
</script>

<div
  class="thread-options-taskbar"
  transition:slide={{ duration: 150, axis: 'y' }}
>
  <div class="taskbar-content">
    <div class="thread-info">
      <h3 class="thread-name">{thread.name}</h3>
      <button class="close-button" onclick={closeMenu} aria-label="Close">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          height="1.2rem"
          viewBox="0 -960 960 960"
          width="1.2rem"
          fill="currentColor"
        >
          <path
            d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"
          />
        </svg>
      </button>
    </div>

    <div class="taskbar-actions">
      <button
        class="action-button"
        class:active={thread.pinned}
        title={thread.pinned
          ? $t('thread.options.unpin')
          : $t('thread.options.pin')}
        onclick={handleTogglePin}
      >
        {#if thread.pinned}
          <svg
            xmlns="http://www.w3.org/2000/svg"
            height="1.2rem"
            viewBox="0 -960 960 960"
            width="1.2rem"
            fill="currentColor"
            ><path
              d="m640-480 80 80v80H520v240l-40 40-40-40v-240H240v-80l80-80v-280h-40v-80h400v80h-40v280Z"
            /></svg
          >
          <span>{$t('thread.options.unpin')}</span>
        {:else}
          <svg
            xmlns="http://www.w3.org/2000/svg"
            height="1.2rem"
            viewBox="0 -960 960 960"
            width="1.2rem"
            fill="currentColor"
            ><path
              d="m640-480 80 80v80H520v240l-40 40-40-40v-240H240v-80l80-80v-280h-40v-80h400v80h-40v280Zm-286 80h252l-46-46v-314H400v314l-46 46Zm126 0Z"
            />
          </svg>
          <span>{$t('thread.options.pin')}</span>
        {/if}
        <div class="shortcut-hint">
          <KeyboardShortcut primary secondary key="P" />
        </div>
      </button>
      {#if hasLinkedThreads}
        <button
          class="action-button"
          title={$t('thread.options.linked')}
          onclick={handleLinked}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 -960 960 960"
            fill="currentColor"
            ><path
              d="M760-600q-57 0-99-34t-56-86H354q-11 42-41.5 72.5T240-606v251q52 14 86 56t34 99q0 66-47 113T200-40q-66 0-113-47T40-200q0-57 34-99t86-56v-251q-52-14-86-56t-34-98q0-66 47-113t113-47q56 0 98 34t56 86h251q14-52 56-86t99-34q66 0 113 47t47 113q0 66-47 113t-113 47ZM200-120q33 0 56.5-24t23.5-56q0-33-23.5-56.5T200-280q-32 0-56 23.5T120-200q0 32 24 56t56 24Zm0-560q33 0 56.5-23.5T280-760q0-33-23.5-56.5T200-840q-32 0-56 23.5T120-760q0 33 24 56.5t56 23.5ZM760-40q-66 0-113-47t-47-113q0-66 47-113t113-47q66 0 113 47t47 113q0 66-47 113T760-40Zm0-80q33 0 56.5-24t23.5-56q0-33-23.5-56.5T760-280q-33 0-56.5 23.5T680-200q0 32 23.5 56t56.5 24Zm0-560q33 0 56.5-23.5T840-760q0-33-23.5-56.5T760-840q-33 0-56.5 23.5T680-760q0 33 23.5 56.5T760-680ZM200-200Zm0-560Zm560 560Zm0-560Z"
            /></svg
          >
          <span>{$t('thread.options.linked')}</span>
          <div class="shortcut-hint">
            <KeyboardShortcut primary secondary key="I" />
          </div>
        </button>
      {:else}
        <div class="action-button action-button-empty">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 -960 960 960"
            height="1.2rem"
            fill="currentColor"
            width="1.2rem"
            ><path
              d="M760-600q-57 0-99-34t-56-86H354q-11 42-41.5 72.5T240-606v251q52 14 86 56t34 99q0 66-47 113T200-40q-66 0-113-47T40-200q0-57 34-99t86-56v-251q-52-14-86-56t-34-98q0-66 47-113t113-47q56 0 98 34t56 86h251q14-52 56-86t99-34q66 0 113 47t47 113q0 66-47 113t-113 47ZM200-120q33 0 56.5-24t23.5-56q0-33-23.5-56.5T200-280q-32 0-56 23.5T120-200q0 32 24 56t56 24Zm0-560q33 0 56.5-23.5T280-760q0-33-23.5-56.5T200-840q-32 0-56 23.5T120-760q0 33 24 56.5t56 23.5ZM760-40q-66 0-113-47t-47-113q0-66 47-113t113-47q66 0 113 47t47 113q0 66-47 113T760-40Zm0-80q33 0 56.5-24t23.5-56q0-33-23.5-56.5T760-280q-33 0-56.5 23.5T680-200q0 32 23.5 56t56.5 24Zm0-560q33 0 56.5-23.5T840-760q0-33-23.5-56.5T760-840q-33 0-56.5 23.5T680-760q0 33 23.5 56.5T760-680ZM200-200Zm0-560Zm560 560Zm0-560Z"
            /></svg
          >
          <span>{$t('thread.options.no_linked')}</span>
        </div>
      {/if}
      {#if hasContent}
        <button
          class="action-button"
          title={$t('thread.options.copy')}
          onclick={handleCopyThread}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            height="1.2rem"
            viewBox="0 -960 960 960"
            width="1.2rem"
            fill="currentColor"
            ><path
              d="M360-240q-33 0-56.5-23.5T280-320v-480q0-33 23.5-56.5T360-880h360q33 0 56.5 23.5T800-800v480q0 33-23.5 56.5T720-240H360Zm0-80h360v-480H360v480ZM200-80q-33 0-56.5-23.5T120-160v-560h80v560h440v80H200Zm160-240v-480 480Z"
            /></svg
          >
          <span>{$t('thread.options.copy')}</span>
          <div class="shortcut-hint">
            <KeyboardShortcut primary secondary key="C" />
          </div>
        </button>
      {:else}
        <div class="action-button action-button-empty">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            height="1.2rem"
            viewBox="0 -960 960 960"
            width="1.2rem"
            fill="currentColor"
            ><path
              d="M360-240q-33 0-56.5-23.5T280-320v-480q0-33 23.5-56.5T360-880h360q33 0 56.5 23.5T800-800v480q0 33-23.5 56.5T720-240H360Zm0-80h360v-480H360v480ZM200-80q-33 0-56.5-23.5T120-160v-560h80v560h440v80H200Zm160-240v-480 480Z"
            /></svg
          >
          <span>{$t('thread.options.no_content')}</span>
        </div>
      {/if}
      <button
        class="action-button"
        title={$t('thread.options.remove')}
        onclick={handleRemoveThread}
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          height="1.2rem"
          viewBox="0 -960 960 960"
          width="1.2rem"
          fill="currentColor"
          ><path
            d="M280-120q-33 0-56.5-23.5T200-200v-520h-40v-80h200v-40h240v40h200v80h-40v520q0 33-23.5 56.5T680-120H280Zm400-600H280v520h400v-520ZM360-280h80v-360h-80v360Zm160 0h80v-360h-80v360ZM280-720v520-520Z"
          /></svg
        >
        <span>{$t('thread.options.remove')}</span>
        <div class="shortcut-hint">
          <KeyboardShortcut primary secondary key="R" />
        </div>
      </button>
    </div>
  </div>
</div>

<style>
  .thread-options-taskbar {
    position: fixed;
    bottom: 0;
    left: 50%;
    transform: translateX(-50%);
    background-color: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 0.5rem 0.5rem 0 0;
    padding: 1rem;
    box-shadow: 0 5px 20px rgba(0, 0, 0, 0.15);
    z-index: 1000;
    backdrop-filter: brightness(60%);
    min-width: 35ch;
    max-width: calc(100% - 2rem);
  }

  .taskbar-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    width: 100%;
  }

  .thread-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border);
  }

  .thread-name {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-main);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .close-button {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--text-muted);
    padding: 0.25rem;
    border-radius: 0.25rem;
    transition: all 0.2s ease;
  }

  .close-button:hover {
    background: color-mix(in srgb, var(--error), transparent 90%);
    color: var(--error);
  }

  .taskbar-actions {
    display: flex;
    justify-content: center;
    gap: 1.5rem;
    overflow-x: auto;
    margin: 0;
  }

  .action-button,
  .action-button-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 1rem;
    background-color: color-mix(in srgb, var(--bg-base), transparent 60%);
    border: none;
    border-radius: 0.25rem;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 0.15s cubic-bezier(0.2, 0, 0, 1);
    overflow: hidden;
    flex: 1 1 0;
    min-width: 0;
    position: relative;
  }

  .action-button-empty {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .action-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
  }

  .action-button svg {
    height: 1.75rem;
    width: 1.75rem;
  }

  .action-button span {
    font-size: 0.8rem;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-align: center;
  }

  .shortcut-hint {
    display: flex;
    align-items: center;
    gap: 0.2rem;
    background-color: var(--bg-main);
    padding: 0.1rem 0.3rem;
    border-radius: 0.15rem;
    font-family: var(--font-mono);
    font-size: 0.7rem;
    color: var(--text-muted);
  }

  @media (max-width: 768px) {
    .thread-options-taskbar {
      min-width: 100%;
      max-width: none;
    }

    .taskbar-actions {
      gap: 0.5rem;
    }

    .action-button {
      padding: 0.75rem;
      min-width: 60px;
    }

    .action-button span {
      font-size: 0.8rem;
    }
  }
</style>
