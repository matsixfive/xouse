<script lang="ts">
	import { onDestroy, onMount } from "svelte";
	import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";

	import {
		Config,
		type ActionType,
		type ConfigType,
		BasicAction,
		ClickAction,
		KeypressAction,
	} from "./bindings";
	import { invoke } from "@tauri-apps/api/core";
	import { ZodError } from "zod";

	let speed = 50;

	let listeners: Promise<UnlistenFn>[] = [];

	let config: ConfigType;

	$: console.log(config);

	// $: {if (config) config.actions = cfg}

	onMount(async () => {
		invoke("get_speed").then((value: unknown) => {
			if (value && typeof value === "number") speed = value;
		});

		console.log("getting config");
		invoke("get_config").then((v: unknown) => {
			console.log("got config");
			console.log(v);
			try {
				const c = Config.parse(v);
				if (!c) return;
				config = c;
			} catch (e) {
				if (e instanceof ZodError) {
					console.log("zod error");
					console.error(e.format());
				} else {
					console.log("error");
					console.error(e);
				}
			}
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

	const getActionType = (action: ActionType) => {
		if (BasicAction.safeParse(action).success) return BasicAction;
		if (ClickAction.safeParse(action).success) return ClickAction;
		if (KeypressAction.safeParse(action).success) return KeypressAction;
		return null;
	};
</script>

<form
	on:submit|preventDefault={() => {
		emit("save_config");
	}}
>
	<input
		type="range"
		min="0"
		max="100"
		step="1"
		on:input={(e) => {
			invoke("update_taskbar_progress", {
				progress: parseInt(e.currentTarget.value) / 100,
			});
		}}
	/>
	<button
		on:click={() => {
			invoke("clear_taskbar_progress");
		}}>Reset</button
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
	{#if config}
		{#each Object.entries(config.actions) as [button, actions]}
			<p>{button}</p>
			{#each actions as action}
				{action}
			{/each}
		{/each}
	{/if}
	<!-- {#each buttonKeys as button} -->
	<!-- 	<Binding {button} bind:action={cfg[button][0]} /> -->
	<!-- {/each} -->
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
