<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { SlideToggle } from "@skeletonlabs/skeleton";

	import type { PageData } from "./$types";
	import { goto } from "$app/navigation";

	export let data: PageData;

	let front = data.note.front;
	let back = data.note.back;
	let deckId = data.params.deckId;
	let noteId = data.params.noteId
	let preview = "";
	let showBack = true;

	async function createCard() {
		await invoke("update_note", {
			deckId,
			noteId,
			front,
			back,
		});
		front = "";
		back = "";
		goto('/decks/' + deckId);
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
		Deck: {deckId}
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
			Update
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
