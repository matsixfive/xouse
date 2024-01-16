<script lang="ts">
    import { createEventDispatcher } from "svelte";

	export let values: [string, string][] = [["none", ""]];

	const dispach = createEventDispatcher();

	export let selected: string = values[0][0];
	$: dispach("input", selected);
</script>

<div class="dropdown">
	<select name="button" bind:value={selected}>
		{#each values as [key, value]}
			<option value={key}>{value}</option>
		{/each}
	</select>
</div>
