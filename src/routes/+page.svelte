<script lang="ts">
import { onMount } from "svelte";
import { config } from "$lib";
import FolderSelector from "$lib/components/FolderSelector.svelte";

onMount(async () => {
	await config.load();
});
</script>

<FolderSelector />