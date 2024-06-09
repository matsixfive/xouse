<script lang="ts">
	import DropDown from "./DropDown.svelte";
	import { buttons, actions } from "./bindings";
	import type { Action } from "./bindings";

	export let action: Action;
	let button: string = "";

	const handleInput = (e: CustomEvent<string>) => {
		button = e.detail;
	};
</script>

<div class="binding">
	<div class="action">{actions[action]}</div>
	<DropDown values={Object.entries(buttons)} on:input={handleInput} />
</div>

<style lang="scss">
	.binding {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
	}

	.action {
		font-weight: 400;
	}
</style>
