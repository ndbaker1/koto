<script lang="ts">
	import { name } from '$lib/constants';
</script>

<svelte:head>
	<title>{name}</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1>About {name}</h1>

	<pre>npm create svelte@latest</pre>

	<p>
		The page you're looking at is purely static HTML, with no client-side interactivity needed.
		Because of that, we don't need to load any JavaScript. Try viewing the page's source, or opening
		the devtools network panel and reloading.
	</p>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}
</style>
