<script lang="ts">
	import DropDown from "./DropDown.svelte";
    import type { AType } from "./bindings";
	// import { buttons, actions as allActions } from "./bindings";
	// import type { AType, Button } from "./bindings";

	export let button: any;
	export let action: AType;

	const handleInput = (e: CustomEvent) => {
		console.log(button, e.detail);
	};
</script>

<div class="binding">
	<div class="button">
		<img
			src={`buttons/xbox/colored/${button.toLowerCase()}.svg`}
			alt={button}
			class="buttonIcon"
		/>
		<!-- <span class="buttonName">{buttons[button]}</span> -->
	</div>
	<div>
		<!-- <DropDown -->
		<!-- 	values={Object.entries(allActions)} -->
		<!-- 	on:input={handleInput} -->
		<!-- 	bind:selected={action} -->
		<!-- /> -->
	</div>
</div>

<style lang="scss">
	.binding {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;

		background-color: #333;
		border-radius: 9999px;

		> * {
			padding: 0.3em;
		}
	}

	.button {
		display: flex;
		align-items: center;
		gap: 0.5em;
	}

	.buttonName {
		font-weight: 400;
	}

	.buttonIcon {
		aspect-ratio: 1 / 1;
		height: 3em;
	}
</style>
