<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	import type { PageData } from "./$types";

	export let data: PageData;
	console.log(data.cards)
</script>

<div class="flex justify-center">
	<h2>{data.params.deck}</h2>
</div>


{#each data.cards as card}
<p>{card.front}</p>
{/each}
		
<style>
</style>
