export const buttons = {
	None: "None",
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

export const actions = {
	lClick: "Left Click",
	rClick: "Right Click",
	mClick: "Middle Click",
	speedUp: "Speed Up",
	speedDown: "Slow Down",
	speedInc: "Add Speed",
	speedDec: "Subtract Speed"
} as const;
export type Action = keyof typeof actions;
export const actionKeys = Object.keys(actions) as Action[];
