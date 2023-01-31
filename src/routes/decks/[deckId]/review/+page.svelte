<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	import type { PageData } from "./$types";

	export let data: PageData;
	
	let { cards } = data;

	let preview: any;

	const renderCard = async (card: any) => await invoke("render_card", { card, back: true });

	$: renderCard(cards[0]).then((p) => (preview = p));
</script>


{@html preview}


{#each data.cards as card}
	<p>{card.note_id}</p>
{/each}

<style>
</style>
