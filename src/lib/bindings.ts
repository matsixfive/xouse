export const buttons = {
	none: "",
	south: "A",
	east: "B",
	west: "X",
	north: "Y",
	dUp: "Dpad Up",
	dDown: "Dpad Down",
	dLeft: "Dpad Left",
	dRight: "Dpad Right",
	lBumper: "Left Bumper",
	rBumper: "Right Bumper",
	lTrigger: "Left Trigger",
	rTrigger: "Right Trigger",
	lStick: "Left Stick",
	rStick: "Right Stick",
	start: "Start",
	select: "Select",
} as const;
export type Button = keyof typeof buttons;

export const actions = { lClick: "Left Click", rClick: "Right Click" } as const;
export type Action = keyof typeof actions;
