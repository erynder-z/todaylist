import type { PopupType } from "./ui";

export type AppState = {
	todayNotePath: string | null;
	todayNoteContent: string | null;
	activePopup: PopupType;
};
