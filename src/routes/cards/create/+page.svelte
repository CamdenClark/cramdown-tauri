<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { SlideToggle } from "@skeletonlabs/skeleton";

	import type { PageData } from "./$types";

	export let data: PageData;

	let front = "";
	let back = "";
	let deck = "";
	let preview = "";
	let showBack = true;

	async function createCard() {
		await invoke("create_note", {
			deck,
			front,
			back,
		});
		front = "";
		back = "";
	}
	async function previewCard(
		showBack: boolean,
		front: string,
		back: string
	): Promise<string> {
		return await invoke("preview_note", {
			showBack,
			card: { front, back },
		});
	}
	$: previewCard(showBack, front, back).then((p) => (preview = p));
</script>

<div class="note-editor">
	<form>
		<label>
			Deck:
			<select bind:value={deck}>
				{#each data.decks as deck}
					<option>{deck}</option>
				{/each}
			</select>
		</label>
		<label>
			Front:
			<textarea id="front-input" bind:value={front} />
		</label>
		<label>
			Back:
			<textarea id="back-input" bind:value={back} />
		</label>
		<button
			class="btn btn-filled-primary btn-base"
			on:click={createCard}
		>
			Create
		</button>
	</form>
	<div class="note-preview">
		<SlideToggle bind:checked={showBack}>Show back</SlideToggle>
		<div>{@html preview}</div>
	</div>
</div>

<style>
	.note-editor {
		display: flex;
		justify-content: space-around;
	}
	.field-inputs {
		min-width: 400px;
		display: flex;
		flex-direction: column;
	}
</style>
