<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	import Config from './lib/Config.svelte';
</script>

<main>
	<Config />
</main>

<style lang="scss">
	main {
		overflow-y: scroll;
		height:100vh;
	}
</style>
