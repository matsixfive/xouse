import { z } from "zod";

export const buttons = {
	South: "A",
	East: "B",
	West: "X",
	North: "Y",
	DPadUp: "DPad Up",
	DPadDown: "DPad Down",
	DPadLeft: "DPad Left",
	DPadRight: "DPad Right",
	LeftTrigger2: "Left Bumper",
	RightTrigger2: "Right Bumper",
	LeftTrigger: "Left Trigger",
	RightTrigger: "Right Trigger",
	LeftThumb: "Left Stick",
	RightThumb: "Right Stick",
	Start: "Start",
	Select: "Select",
} as const;
export type Button = keyof typeof buttons;
export const buttonKeys = Object.keys(buttons) as Button[]

export const actions = {
	None: "None",
	LClick: "Left Click",
	RClick: "Right Click",
	MClick: "Middle Click",
	SpeedUp: "Speed Up",
	SpeedDown: "Slow Down",
	SpeedInc: "Add Speed",
	SpeedDec: "Subtract Speed",
	Rumble: "Rumble",
	KeyPress: "Key Press",
} as const;
export type Action = keyof typeof actions;
export const actionKeys = Object.keys(actions) as Action[];

export type A = (Action | { KeyPress: [string, string[]] })[]

const Ac = z.union(
	[
		z.enum(["None", "LClick", "RClick", "MClick", "SpeedUp", "SpeedDown", "SpeedInc", "SpeedDec", "Rumble"]),
		z.object(
			{
				KeyPress: z.tuple([
					z.string(),
					z.array(z.enum(["Shift", "Ctrl", "Alt", "Meta"]))
				])
			}
		)
	]
).array();

export type AType = z.infer<typeof Ac>;

export const Config = z.object({
	speed: z.number(),
	speed_down: z.number(),
	speed_up: z.number(),
	speed_inc: z.number(),
	actions: z.object({
		South: Ac,
		East: Ac,
		West: Ac,
		North: Ac,
		DPadUp: Ac,
		DPadDown: Ac,
		DPadLeft: Ac,
		DPadRight: Ac,
		LeftTrigger2: Ac,
		RightTrigger2: Ac,
		LeftTrigger: Ac,
		RightTrigger: Ac,
		LeftThumb: Ac,
		RightThumb: Ac,
		Start: Ac,
		Select: Ac,
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
