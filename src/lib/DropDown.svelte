<script lang="ts">
	import { createEventDispatcher } from "svelte";

	export let values: [string, string][] = [["none", ""]];

	export let selected: string = values[0][0];

	const dispach = createEventDispatcher();
	$: dispach("input", selected);
</script>

<div class="dropdown">
	<select class="actionSelect" name="action" bind:value={selected}>
		{#each values as [key, value]}
			<option value={key}>{value}</option>
		{/each}
	</select>
</div>

<style lang="scss">
	.actionSelect {
		appearance: none;
		width: 100%;
		font-size: 1.15rem;
		padding: 0.675em 6em 0.675em 1em;
		border: none;
		border-radius: 9999px;
		background-color: #222;
		cursor: pointer;
	}
</style>
