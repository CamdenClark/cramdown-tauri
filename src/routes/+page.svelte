<script lang="ts">
  import Greet from "$lib/Greet.svelte";
  import type { PageData } from "./$types";
  export let data: PageData;
</script>

<h1>Welcome to Tauri!</h1>


<div class="row">
  <Greet />
</div>

{#each data.decks as deck}
<div><a href='/decks/{deck}'>{deck}</div>
{/each}
