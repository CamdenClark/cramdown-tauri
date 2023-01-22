<script>
	import '@skeletonlabs/skeleton/themes/theme-rocket.css';
	import "@skeletonlabs/skeleton/styles/all.css";
	import "../app.postcss";

	import { AppShell } from "@skeletonlabs/skeleton";
</script>

<AppShell>
	<svelte:fragment slot="header"><h1><a href='/'>cramdown</a></h1></svelte:fragment>
	<svelte:fragment slot="pageHeader">Page Header</svelte:fragment>
	<!-- Router Slot -->
	<slot />
	<!-- ---- / ---- -->
	<svelte:fragment slot="footer">Footer</svelte:fragment>
</AppShell>
