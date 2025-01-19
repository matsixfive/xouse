<script lang="ts" async>
	import { attachConsole } from "@tauri-apps/plugin-log";
	const detach = attachConsole();

	import Config from "./lib/Config.svelte";
</script>

<main>
	<Config />
</main>

<style lang="scss">
	main {
		overflow-y: scroll;
		height: 100vh;
	}
</style>
