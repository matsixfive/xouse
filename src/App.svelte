<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	let speed = 6;
	onMount( async () => {
		listen("speed-up", ({payload}) => {
			console.log(`Speeding up to ${payload}!`);
			speed = payload;
			});
		listen("speed-down", ({payload}) => {
			console.log(`Slowing down to ${payload}!`);
			speed = payload;
			});
	});
</script>

<main>
	<h1>Speed: {speed}</h1>
</main>
