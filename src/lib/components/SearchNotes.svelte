<script lang="ts">
  /**
   * Note search component.
   */
  import { t } from '$lib';
</script>

<div class="search-notes">
  <div class="input-container">
    <!-- svelte-ignore a11y_autofocus -->
    <input
      type="text"
      placeholder={$t('search.title')}
      spellcheck="false"
      autofocus
    />
  </div>

  <div class="status-section">
    <p class="muted">Search functionality coming soon!</p>
  </div>
</div>

<style>
  .search-notes {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    width: 100%;
  }

  .input-container {
    position: relative;
    display: flex;
    align-items: center;
  }

  input {
    width: 100%;
    background: color-mix(in srgb, var(--accent), transparent 80%);
    border: none;
    color: var(--text-main);
    padding: 0.75rem 1rem;
    font-size: 1rem;
    outline: none;
    border-radius: 0.5rem;
  }

  input:focus {
    border-color: var(--accent);
  }

  .status-section {
    padding: 2rem;
    text-align: center;
    color: var(--text-muted);
    background-color: var(--bg-surface);
    border-radius: 0.5rem;
    border: 1px dashed var(--border);
    font-size: 0.9rem;
  }

  .muted {
    margin: 0;
  }
</style>
