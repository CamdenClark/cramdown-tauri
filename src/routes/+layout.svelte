<script>
	import "@skeletonlabs/skeleton/themes/theme-skeleton.css";
	import "@skeletonlabs/skeleton/styles/all.css";
	import "../app.postcss";

	import { AppShell } from "@skeletonlabs/skeleton";
</script>

<AppShell>
	<svelte:fragment slot="header"><h1>cramdown</h1></svelte:fragment>
	<svelte:fragment slot="pageHeader">Page Header</svelte:fragment>
	<!-- Router Slot -->
	<slot />
	<!-- ---- / ---- -->
	<svelte:fragment slot="footer">Footer</svelte:fragment>
</AppShell>
