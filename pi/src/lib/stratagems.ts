import data from "./stratagems.json";

export type Direction = "up" | "down" | "left" | "right";

export type Stratagem = {
	/** Stored as the action setting "stratagem". */
	id: string;
	title: string;
	/** Path inside the plugin folder without ".png", same as the manifest's "Icon". */
	icon: string;
	seq: Direction[];
};

/** Keyed by group name - the last segment of the action UUID. */
export const stratagemGroups = data as Record<string, Stratagem[]>;

/** The PI is served from <plugin>/pi/, the icons from <plugin>/icon/. */
export const iconUrl = (icon: string) => `../${icon}.png`;

/** Shown when no stratagem is selected - same image the plugin puts on the key. */
export const PLACEHOLDER_ICON = "icon/helldivers2";
