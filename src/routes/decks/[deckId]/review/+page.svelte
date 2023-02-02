<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	enum CardScore {
		Again = "Again",
		Hard = "Hard",
		Good = "Good",
		Easy = "Easy",
	}

	import type { PageData } from "./$types";

	export let data: PageData;

	let { cards } = data;
	let back = false;

	let preview: any;

	const renderCard = async (card: any, back: boolean) =>
		await invoke("render_card", { card, back });

	const showBack = () => (back = true);

	const submitReview = async (score: CardScore) => {
		console.log(cards);
		const [card, ...newCards] = cards;
		cards = newCards;
		back = false;
		await invoke("review_card", { card, score });
	};

	$: renderCard(cards[0], back).then((p) => (preview = p));
</script>

<div class="flashcard p-3 text-xl">
	{@html preview}
</div>

<div>
	{#if back}
		<button
			class="btn btn-base btn-filled-primary"
			on:click={() => submitReview(CardScore.Again)}
		>
			Again
		</button>
		<button
			class="btn btn-base btn-filled-primary"
			on:click={() => submitReview(CardScore.Hard)}
		>
			Hard
		</button>
		<button
			class="btn btn-base btn-filled-primary"
			on:click={() => submitReview(CardScore.Good)}
		>
			Good
		</button>
		<button
			class="btn btn-base btn-filled-primary"
			on:click={() => submitReview(CardScore.Easy)}
		>
			Easy
		</button>
	{:else}
		<button
			class="btn btn-base btn-filled-primary"
			on:click={showBack}>Show back</button
		>
	{/if}
</div>

<style>
	.flashcard :global(p) {
		@apply text-xl;
	}
</style>
