<script lang="ts">
	import { onMount } from "svelte";
	import { emit, listen } from "@tauri-apps/api/event";
    import Binding from "./Binding.svelte";

	let speed = 50;

	onMount(async () => {
		listen("speed-change", ({ payload }: { payload: number }) => {
			console.log(`Speed: ${payload}!`);
			speed = payload;
		});
	});

	const updateSpeed = async (value: number) => {
		if (value) {
			speed = value;
			emit("speed-change", value);
		}
	};
</script>

<form on:submit|preventDefault={() => {emit("save-config")}}>
	<h1>
		Speed:
		<input
			class="num-input"
			type="number"
			on:input={(e) => {
				updateSpeed(parseInt(e.currentTarget.value));
			}}
			bind:value={speed}
			min="0"
			max="100"
			step="5"
		/>
	</h1>
	<input type="submit" value="Save" />
</form>
<Binding action="lClick" />
<Binding action="rClick" />

<style lang="scss">
	.num-input {
		all: unset;
		font-weight: 400;
		display: inline-block;
		border: none;
		border-radius: 0.5rem;
		padding: 0.5rem;
		background: transparent;
		min-width: 0px;
		width: auto;

		&:focus,
		&:active,
		&:hover {
			outline: 1px solid #fff;
		}
	}
</style>
