<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	import type { PageData } from "./$types";

	export let data: PageData;

	let front = "";
	let back = "";
	let preview = "";
	let showBack = true;

	async function createCard() {
		await invoke("create_note", {
			deck: data.params.deck,
			front,
			back,
		});
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
	<div class="field-inputs">
		<textarea id="front-input" bind:value={front} />
		<textarea id="back-input" bind:value={back} />
		<button on:click={createCard}>Create</button>
	</div>
	<div class="note-preview">
		<label for="showBack">Show back: <input id="showBack" type="checkbox" bind:checked={showBack} /></label>
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
