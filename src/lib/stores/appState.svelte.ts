import type { AppState } from "$lib/types/appState";

export const appState = $state<AppState>({
	todayNotePath: null,
	todayNoteContent: null,
	activePopup: null,
});
