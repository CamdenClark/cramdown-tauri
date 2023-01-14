<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	import type { PageData } from "./$types";

	export let data: PageData;

	let front = "";
	let back = "";

	async function createCard() {
		await invoke("create_card", { deck: data.params.deck, front, back });
	}

</script>

<div>
	<div class="row">
		<input
			id="deck-name-input"
			bind:value={front}
		/>
		<input
			id="deck-name-input"
			bind:value={back}
		/>
		<button on:click={createCard}>Create</button>
	</div>

</div>

<style>
</style>
