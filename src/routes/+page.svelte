<script lang="ts">
	import type { PageData } from "./$types";
	export let data: PageData;
</script>


<div class="flex justify-center"><a href="/review">Review</a></div>

<div class="flex justify-center"><a href="/cards/create">Create a card</a></div>

<div class="flex">
<div class="flex mx-auto flex-col items-start bg-gray-100 p-2">
        Decks
	{#each data.decks as deck}
		<div class="text-primary-300"><a href="/decks/{deck}">{deck}</a></div>
	{/each}
	<div class="mt-8"><a href="/decks/create">Create new deck</a></div>

</div>
</div>
