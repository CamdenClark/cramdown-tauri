<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	import type { PageData } from "./$types";

	export let data: PageData;
</script>

<div class="flex justify-center">
	<h2>{data.params.deck}</h2>
</div>

<a href='/decks/{data.params.deck}/review'>Review</a>

{#each data.notes as note}
	<p><a href='/decks/{data.params.deck}/notes/{note.note_id}'>{note.note_id}</a></p>
{/each}

<style>
</style>
