import type { AppState } from "$lib/types/appState";

/**
 * Global application state including current note information and UI state.
 */
export const appState = $state<AppState>({
	/** Absolute path to the currently active note. */
	todayNotePath: null,
	/** Full text content of the active note. */
	todayNoteContent: null,
	/** ID of the currently active modal or popup. */
	activePopup: null,
});
