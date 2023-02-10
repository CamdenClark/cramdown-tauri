<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { SlideToggle } from "@skeletonlabs/skeleton";

	import type { PageData } from "./$types";

	export let data: PageData;

	let front = "";
	let back = "";
	let deck = "";
	let preview = "";
	let template = "basic";
	let showBack = true;

	async function createCard() {
		await invoke("create_note", {
			note: {
				deck_id: deck,
				note_id: "",
				template: "basic",
			},
			fields: {
				Front: front,
				Back: back,
			},
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
			fields: { Front: front, Back: back },
			template: "basic",
			cardNum: 0,
		});
	}
	$: previewCard(showBack, front, back).then((p) => (preview = p));
</script>

<div class="note-editor">
	<form on:submit={createCard}>
		<label>
			Deck:
			<select bind:value={deck} required>
				{#each data.decks as deck}
					<option>{deck}</option>
				{/each}
			</select>
		</label>
		<label>
			Template:
			<select bind:value={template} required>
				<option>basic</option>
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
		<button class="btn btn-filled-primary btn-base">
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
