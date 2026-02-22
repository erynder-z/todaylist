<script lang="ts">
  import { appState } from '$lib';

  let { title, children } = $props();

  const close = () =>
    appState.update((state) => ({ ...state, activePopup: null }));
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="overlay" onclick={close}>
  <div class="popup" onclick={(e) => e.stopPropagation()}>
    <div class="popup-header">
      {#if title}<h2>{title}</h2>{/if}
      <button onclick={close} class="close-button">×</button>
    </div>
    <div class="popup-content">
      {@render children()}
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.6);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 2000;
    backdrop-filter: blur(2px);
  }

  .popup {
    background-color: #f6f6f6;
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
    position: relative;
    max-width: 90vw;
    max-height: 85vh;
    overflow-y: auto;
    width: 500px;
  }

  .popup-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    padding-bottom: 0.5rem;
  }

  h2 {
    margin: 0;
    font-size: 1.2rem;
    color: #333;
  }

  .close-button {
    background: none;
    border: none;
    font-size: 1.8rem;
    cursor: pointer;
    color: #666;
    line-height: 1;
    padding: 0 0.5rem;
  }

  .close-button:hover {
    color: #646cff;
  }

  @media (prefers-color-scheme: dark) {
    .popup {
      background-color: #2f2f2f;
      color: #f6f6f6;
    }

    h2 {
      color: #f6f6f6;
    }

    .popup-header {
      border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    }

    .close-button {
      color: #aaa;
    }
  }
</style>
