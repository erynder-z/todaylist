<script lang="ts">
  import { appState, FolderSelector, Modal, NotesList, SearchNotes } from '$lib';
</script>

{#if $appState.activePopup === 'folderSelector'}
  <Modal title="Settings">
    <FolderSelector />
  </Modal>
{:else if $appState.activePopup === 'notesList'}
  <Modal title="Your Notes">
    <NotesList />
  </Modal>
{:else if $appState.activePopup === 'search'}
  <Modal title="Search">
    <SearchNotes />
  </Modal>
{/if}
