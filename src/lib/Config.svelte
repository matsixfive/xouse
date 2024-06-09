<script lang="ts">
	import { onDestroy, onMount } from "svelte";
	import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
	import Binding from "./Binding.svelte";
	import { buttonKeys, Config, type A, type ConfigType } from "./bindings";
	import { invoke } from "@tauri-apps/api";

	let speed = 50;

	let listeners: Promise<UnlistenFn>[] = [];

	let config: ConfigType;
	let cfg: Record<string, A> = {
		South: ["None"],
		East: ["None"],
		West: ["None"],
		North: ["None"],
		DPadUp: ["None"],
		DPadDown: ["None"],
		DPadLeft: ["None"],
		DPadRight: ["None"],
		LeftTrigger2: ["None"],
		RightTrigger2: ["None"],
		LeftTrigger: ["None"],
		RightTrigger: ["None"],
		LeftThumb: ["None"],
		RightThumb: ["None"],
		Start: ["None"],
		Select: ["None"],
	};

	$: console.log(config);
	$: console.log(cfg);

	// $: {if (config) config.actions = cfg}

	onMount(async () => {
		invoke("get_speed").then((value: unknown) => {
			if (value && typeof value === "number") speed = value;
		});

		invoke("get_config").then((v: unknown) => {
			const c = Config.parse(v);
			if (!c) return;
			config = c;
			cfg = config.actions;
		});

		listeners.push(
			listen("speed_change", ({ payload }: { payload: number }) => {
				if (payload === speed) return;
				console.log(`Speed: ${payload}!`);
				speed = payload;
			}),
		);
	});

	onDestroy(() => {
		listeners.forEach((listener) => {
			listener.then((unlisten) => unlisten());
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
			class="numInput"
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
<div class="mappings">
	{#each buttonKeys as button}
		<Binding {button} bind:action={cfg[button][0]} />
	{/each}
</div>

<style lang="scss">
	.numInput {
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

	.mappings {
		border-radius: 1em;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		gap: 5px;
	}
</style>
