import { writable } from "svelte/store";
import type { AppState } from "$lib/types/appState";

const initialAppState: AppState = {
	todayNotePath: null,
	todayNoteContent: null,
	activePopup: null,
};

export const appState = writable<AppState>(initialAppState);
