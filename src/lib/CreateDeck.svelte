<script lang="ts">
	import { goto } from "$app/navigation";

	import { invoke } from "@tauri-apps/api/tauri";

	let name = "";

	async function createDeck() {
		await invoke("create_deck", { name });
		goto("/");
	}
</script>

<div class="flex justify-center">
	<form>
		<label for="deckname">
			<span>Deck name:</span>
			<input
				id="deck-name-input"
				name="deckname"
				placeholder="Enter a deck name..."
				bind:value={name}
				required
			/>
		</label>
		<button
			class="btn btn-filled-primary mt-8"
			on:click={createDeck}
		>
			Create Deck
		</button>
	</form>
</div>
