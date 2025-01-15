import { z } from "zod";

// export const buttons = {
// 	South: "A",
// 	East: "B",
// 	West: "X",
// 	North: "Y",
// 	DPadUp: "DPad Up",
// 	DPadDown: "DPad Down",
// 	DPadLeft: "DPad Left",
// 	DPadRight: "DPad Right",
// 	LeftTrigger2: "Left Bumper",
// 	RightTrigger2: "Right Bumper",
// 	LeftTrigger: "Left Trigger",
// 	RightTrigger: "Right Trigger",
// 	LeftThumb: "Left Stick",
// 	RightThumb: "Right Stick",
// 	Start: "Start",
// 	Select: "Select",
// } as const;
// export type Button = keyof typeof buttons;
// export const buttonKeys = Object.keys(buttons) as Button[]
//
// export const actions = {
// 	None: "None",
// 	LClick: "Left Click",
// 	RClick: "Right Click",
// 	MClick: "Middle Click",
// 	SpeedUp: "Speed Up",
// 	SpeedDown: "Slow Down",
// 	SpeedInc: "Add Speed",
// 	SpeedDec: "Subtract Speed",
// 	Rumble: "Rumble",
// 	KeyPress: "Key Press",
// } as const;
// export type Action = keyof typeof actions;
// export const actionKeys = Object.keys(actions) as Action[];
//
// export type A = (Action | { KeyPress: [string, string[]] })[]

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

// export type Config = {
// 	speed: number,
// 	speed_down: number,
// 	speed_up: number,
// 	speed_inc: number,
// 	speed_dec: number,
// 	actions: {
// 		South?: A[],
// 		East?: A[],
// 		West?: A[],
// 		North?: A[],
// 		DPadUp?: A[],
// 		DPadDown?: A[],
// 		DPadLeft?: A[],
// 		DPadRight?: A[],
// 		LeftTrigger2?: A[],
// 		RightTrigger2?: A[],
// 		LeftTrigger?: A[],
// 		RightTrigger?: A[],
// 		LeftThumb?: A[],
// 		RightThumb?: A[],
// 		Start?: A[],
// 		Select?: A[],
// 	}
// };
