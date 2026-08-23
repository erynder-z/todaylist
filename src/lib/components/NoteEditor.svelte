<script lang="ts">
  /**
   * The main note editor component. Coordinates the Milkdown instance with the EditorStore.
   * Handles high-level actions like shortcuts and thread navigation.
   */
  import type { Editor } from '@milkdown/kit/core';
  import { keymap } from '@milkdown/kit/prose/keymap';
  import { $prose as prosePlugin } from '@milkdown/kit/utils';
  import { tick } from 'svelte';
  import type { NoteContentResponse, NoteThread } from '$lib/interfaces/notes';
  import { tagSuggestionShortcuts } from '../config/shortcuts';
  import { linkOpenerPlugin } from '../plugins/linkOpenerPlugin';
  import type { EditorStore } from '../stores/editor.svelte';
  import { sessionState } from '../stores/sessionState.svelte';
  import { settings } from '../stores/settings.svelte';
  import {
    navigateToLastAvailable,
    navigateToOffset,
  } from '../utils/dailyNote';
  import { EditorService } from '../utils/editor';
  import { useShortcuts } from '../utils/shortcuts';
  import FloatingToolbar from './FloatingToolbar.svelte';
  import MilkdownEditor from './MilkdownEditor.svelte';

  let {
    noteContent = $bindable(),
    notePath,
    editor,
  } = $props<{
    noteContent: NoteContentResponse | null;
    notePath: string | null;
    editor: EditorStore;
  }>();

  let milkdownInstance: Editor | null = $state(null);
  let editorService: EditorService | null = $state(null);

  let lastNotePath = $state<string | null>(null);
  let isTransitioning = $state<boolean>(false);
  let transitionTimeout: ReturnType<typeof setTimeout> | null = null;

  $effect(() => {
    return () => {
      if (transitionTimeout) clearTimeout(transitionTimeout);
    };
  });

  // Disable built-in milkdown keymaps
  const customKeymap = prosePlugin(() =>
    keymap({
      'Mod-Alt-1': () => true,
      'Mod-Alt-2': () => true,
      'Mod-Alt-3': () => true,
      'Mod-Alt-4': () => true,
      'Mod-Alt-5': () => true,
      'Mod-Alt-6': () => true,
      'Mod-b': () => true,
      'Mod-i': () => true,
      'Mod-e': () => true,
      'Mod-x`': () => true,
    }),
  );
  const stablePlugins = $derived([customKeymap, linkOpenerPlugin]);

  /**
   * Update editor service when milkdown instance changes.
   */
  $effect(() => {
    editorService = milkdownInstance
      ? new EditorService(milkdownInstance)
      : null;
  });

  /**
   * Sync props to the internal store before rendering
   */
  $effect.pre(() => {
    editor.sync(noteContent, notePath);
  });

  /**
   * Coordinate reactive updates
   * Reacts to editor content changes and editor service availability
   */
  $effect(() => {
    const instance = milkdownInstance;
    const service = editorService;
    const hasPendingUpdate = editor.pendingExternalUpdate;
    const pendingJump = sessionState.pendingThreadJump;
    const currentPath = notePath;

    if (!instance || !service || !hasPendingUpdate) return;

    // Clear the flag and apply the update
    editor.pendingExternalUpdate = false;

    const isPathChange = currentPath !== lastNotePath;

    const performUpdate = () => {
      if (isPathChange) {
        const editorElement = document.querySelector('.editor-main');
        if (editorElement) editorElement.scrollTop = 0;
      }

      // If we have a pending thread jump (thread ID), handle it after content update
      if (pendingJump) {
        // Clear the pending jump immediately to avoid re-triggering
        sessionState.pendingThreadJump = null;

        // Update content and jump to thread when complete
        service.updateContent(editor.content, () => {
          editor.jumpToThread(pendingJump);
          if (isPathChange) {
            isTransitioning = false;
          }
        });
      } else {
        // No pending jump, just update content normally
        service.updateContent(editor.content, () => {
          if (isPathChange) isTransitioning = false;
        });
      }
      lastNotePath = currentPath;
    };

    if (isPathChange && lastNotePath !== null) {
      isTransitioning = true;
      if (transitionTimeout) clearTimeout(transitionTimeout);
      transitionTimeout = setTimeout(() => {
        performUpdate();
      }, 150); // Match CSS transition duration
    } else {
      // Immediate update for non-path changes or initial load
      performUpdate();
    }
  });

  /**
   * Auto-focus the editor when no popup is active and we have a note path
   */
  $effect(() => {
    const instance = milkdownInstance;
    const service = editorService;
    if (!instance || !service) return;

    if (sessionState.activePopup === null && notePath) service.focus();
  });

  /**
   * Main entry point for jumping to a thread.
   */
  const handleJump = async (threadId: string) => {
    const instance = milkdownInstance;
    if (!instance || !editorService) return;

    const threadIndex = editor.threads.findIndex(
      (nt: NoteThread) => nt.id === threadId,
    );
    if (threadIndex !== -1) {
      editorService.jumpToThreadByIndex(threadIndex);
    } else {
      tick().then(() => {
        if (milkdownInstance && editorService) {
          const newThreadIndex = editor.threads.findIndex(
            (s: NoteThread) => s.id === threadId,
          );
          if (newThreadIndex !== -1) {
            editorService.jumpToThreadByIndex(newThreadIndex);
          }
        }
      });
    }
  };

  /**
   * When in Navigation Mode: Jumps to a thread based on its index.
   * When in Thread Options Mode: Opens the thread options popup.
   */
  const jumpToThreadByIndex = async (idx: number) => {
    const thread = editor.threads[idx];
    if (thread?.id) {
      if (sessionState.threadShortcutsMode === 'navigation') {
        await handleJump(thread.id);
      } else {
        sessionState.selectedThreadForOptions = thread;
        sessionState.activePopup = 'threadOptions';
      }
    }
  };

  /**
   * Shortcut handler: Focus the last line of the editor
   */
  const handleFocusLastLine = (): boolean => {
    if (sessionState.activePopup !== null) return false;
    if (milkdownInstance && editorService) {
      editorService.focusEnd();
      return true;
    }
    return false;
  };

  /**
   * Shortcut handler: Jump to thread by number shortcut
   */
  const handleJumpByNumber = (e: KeyboardEvent): boolean => {
    if (
      sessionState.activePopup !== null &&
      sessionState.activePopup !== 'threadOptions'
    )
      return false;

    const idx = tagSuggestionShortcuts.codes.indexOf(e.code);
    if (idx !== -1 && idx < editor.threads.length) {
      jumpToThreadByIndex(idx);
      return true;
    }

    return false;
  };

  /**
   * Shortcut handler: Navigate to yesterday's note
   */
  const handleNavigateYesterday = async (e: Event) => {
    if (sessionState.activePopup !== null) {
      e.preventDefault();
      return;
    }

    await navigateToOffset(-1);
  };

  /**
   * Shortcut handler: Navigate to last available note
   */
  const handleNavigateLastAvailable = async (e: Event) => {
    if (sessionState.activePopup !== null) {
      e.preventDefault();
      return;
    }

    await navigateToLastAvailable();
  };

  /**
   * Shortcut handler: Navigate to today's note
   */
  const handleNavigateToday = async (e: Event) => {
    if (sessionState.activePopup !== null) {
      e.preventDefault();
      return;
    }
    await navigateToOffset(0);
  };

  useShortcuts({
    focusLastLine: handleFocusLastLine,
    jumpByNumber: handleJumpByNumber,
    navigateYesterday: handleNavigateYesterday,
    navigateLastAvailable: handleNavigateLastAvailable,
    navigateToday: handleNavigateToday,
  });

  /**
   * Connect the store's sync back to the component's bindable props
   */
  $effect(() => {
    editor.onContentUpdate = (updated: NoteContentResponse) =>
      (noteContent = updated);
  });

  /**
   * Expose jump functionality to parent components
   */
  $effect(() => {
    editor.jumpToThread = (threadId: string) => {
      const instance = milkdownInstance;
      if (instance && editorService) {
        const threadIndex = editor.threads.findIndex(
          (nt: NoteThread) => nt.id === threadId,
        );
        if (threadIndex !== -1) editorService.jumpToThreadByIndex(threadIndex);
      }
    };
  });
</script>

{#if notePath}
  <div
    class="editor-transition-container"
    class:transitioning={isTransitioning}
  >
    <MilkdownEditor
      content={editor.content}
      onReady={(inst) => {
        milkdownInstance = inst;
      }}
      onUpdate={(markdown: string) => editor.updateContent(markdown)}
      plugins={stablePlugins}
      activeThreadIndex={sessionState.activePopup === 'threadOptions'
        ? editor.threads.findIndex(
            (t: NoteThread) =>
              t.id === sessionState.selectedThreadForOptions?.id,
          )
        : -1}
    />
    {#if settings.floatingToolbarEnabled}
      <FloatingToolbar editorInstance={milkdownInstance} />
    {/if}
  </div>
{/if}

<style>
  .editor-transition-container {
    opacity: 1;
    transition: opacity 150ms cubic-bezier(0.4, 0, 0.2, 1);
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .editor-transition-container.transitioning {
    opacity: 0;
    overflow: hidden;
  }
</style>
