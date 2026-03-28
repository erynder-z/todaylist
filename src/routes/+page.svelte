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

  :root {
    font-family: 'SUSE Mono', monospace;
    font-weight: 600;
    background-color: var(--bg-base);
    color: var(--text-main);
    line-height: 1.6;
    -webkit-font-smoothing: antialiased;
  }

  .container {
    margin: 0 auto;
    padding: 3rem 1rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    transition:
      background-color 0.3s,
      color 0.3s;
  }
</style>
