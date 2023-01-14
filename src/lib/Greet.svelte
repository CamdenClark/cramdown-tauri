<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	let name = "";
	let deckName = "";

	async function createDeck() {
		await invoke("create_deck", { name });
	}

</script>

<div>
	<div class="row">
		<input
			id="deck-name-input"
			placeholder="Enter a deck name..."
			bind:value={name}
		/>
		<button on:click={createDeck}> Greet </button>
	</div>

</div>
