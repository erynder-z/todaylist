<script lang="ts">
  /**
   * Application entry point. Handles initial bootstrapping and renders the main UI structure.
   */
  import { onMount } from 'svelte';
  import {
    initializeApp,
    lang,
    locale,
    MainView,
    Navigation,
    PopupManager,
  } from '$lib';

  /**
   * Bootstraps the app by fetching initial state from the backend.
   */
  onMount(async () => {
    await initializeApp();
  });
</script>

<svelte:body use:lang={$locale} />

<main class="container">
  <MainView />
  <Navigation />
  <PopupManager />
</main>

<style>
  @font-face {
    font-family: SUSE Mono;
    src: url(../assets/SUSEMono-VariableFont_wght.ttf) format('truetype');
  }

  :global(:root) {
    box-sizing: border-box;
    font-family: 'SUSE Mono', monospace;
    font-weight: 600;
    background-color: var(--bg-base);
    color: var(--text-main);
    line-height: 1.6;
    -webkit-font-smoothing: antialiased;
  }

  :global(*),
  :global(*::before),
  :global(*::after) {
    box-sizing: inherit;
    margin: 0;
    padding: 0;
  }

  .container {
    height: 100dvh;
    width: 100%;
    margin: 0 auto;
    padding: 3rem 1rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    transition:
      background-color 0.3s,
      color 0.3s;
  }

  @media (max-width: 480px) {
    .container {
      padding: 0;
    }
  }

  :global(::-webkit-scrollbar) {
    width: 8px;
    height: 8px;
  }

  :global(::-webkit-scrollbar-track) {
    background: color-mix(in srgb, var(--bg-base), transparent 50%);
    border-radius: 4px;
  }

  :global(::-webkit-scrollbar-thumb) {
    background: color-mix(in srgb, var(--text-muted), transparent 30%);
    border-radius: 4px;
    border: 2px solid color-mix(in srgb, var(--bg-base), transparent 50%);
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  :global(::-webkit-scrollbar-thumb:hover) {
    background: color-mix(in srgb, var(--text-muted), transparent 20%);
    opacity: 1;
  }

  :global(*:hover::-webkit-scrollbar-thumb) {
    opacity: 1;
  }

  :global(html) {
    scrollbar-width: auto;
  }

  :global(html) {
    scrollbar-width: thin;
    scrollbar-color: transparent transparent;
  }

  :global(*:hover) {
    scrollbar-color: color-mix(in srgb, var(--text-muted), transparent 30%)
      color-mix(in srgb, var(--bg-base), transparent 50%);
  }
</style>
