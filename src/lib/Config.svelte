<script lang="ts">
	import { onMount } from "svelte";
	import { emit, listen } from "@tauri-apps/api/event";
	import Binding from "./Binding.svelte";
	import { actionKeys } from "./bindings";
	import { invoke } from "@tauri-apps/api";

	let speed = 50;

	onMount(async () => {
		invoke("get_speed").then((value: any) => {
			if (value && typeof value === "number") speed = value;
		});

		listen("speed_change", ({ payload }: { payload: number }) => {
			console.log(`Speed: ${payload}!`);
			speed = payload;
		});
	});

	const updateSpeed = async (speed: number) => {
		if (speed) {
			invoke("set_speed", { speed });
		}
	};
</script>

<form
	on:submit|preventDefault={() => {
		emit("save_config");
	}}
>
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
			max="150"
			step="5"
		/>
	</h1>
	<input type="submit" value="Save" />
</form>
{#each actionKeys as action}
	<Binding {action} />
{/each}

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
