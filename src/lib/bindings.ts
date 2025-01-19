import { z } from "zod";

export const BasicAction = z.enum(["None", "SpeedUp", "SpeedDown", "SpeedInc", "SpeedDec", "Rumble", "ToggleVis"]);
export const ClickAction = z.object({ Click: z.enum(["Left", "Middle", "Right"]) });
export const KeypressAction =
	z.object(
		{
			KeyPress: z.tuple([
				z.string(),
				z.array(z.enum(["Shift", "Ctrl", "Alt", "Meta"]))
			])
		}
	);
export type BasicActionType = z.infer<typeof BasicAction>;
export type ClickActionType = z.infer<typeof ClickAction>;
export type KeypressActionType = z.infer<typeof KeypressAction>;

const Action = z.union([BasicAction, KeypressAction, ClickAction,]);

export type ActionType = z.infer<typeof Action>;

export const Config = z.object({
	speed: z.number(),
	speed_up: z.number(),
	speed_down: z.number(),
	speed_step: z.number(),
	actions: z.object({
		South: Action.array(),
		East: Action.array(),
		West: Action.array(),
		North: Action.array(),
		DPadUp: Action.array(),
		DPadDown: Action.array(),
		DPadLeft: Action.array(),
		DPadRight: Action.array(),
		LeftTrigger: Action.array(),
		RightTrigger: Action.array(),
		LeftBumper: Action.array(),
		RightBumper: Action.array(),
		LeftThumb: Action.array(),
		RightThumb: Action.array(),
		Start: Action.array(),
		Select: Action.array(),
	}).partial()
});

export type ConfigType = z.infer<typeof Config>
