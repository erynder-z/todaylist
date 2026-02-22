<script lang="ts">
  export let noteContent: string;
</script>

<div class="note-display">
  <pre>{noteContent}</pre>
</div>

<style>
  .note-display {
    white-space: pre-wrap;
    word-wrap: break-word;
    padding: 1rem;
    background-color: #f9f9f9;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    margin: 1rem 0;
    max-height: 70vh;
    overflow-y: auto;
  }

  pre {
    margin: 0;
    font-family: monospace;
    font-size: 0.9em;
    line-height: 1.4;
  }

  @media (prefers-color-scheme: dark) {
    .note-display {
      background-color: #333;
      border: 1px solid #555;
      color: #eee;
    }
  }
</style>
