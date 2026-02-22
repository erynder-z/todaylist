import { invoke } from "@tauri-apps/api/core";
import { appState, settings } from "$lib";

export const initializeApp = async () => {
	try {
		const initialState: {
			notes_folder: string | null;
			today_note_path: string | null;
			today_note_content: string | null;
		} = await invoke("initialize_app");

		if (initialState.notes_folder) {
			settings.set({ notes_folder: initialState.notes_folder });
		}

		appState.update((state) => ({
			...state,
			todayNotePath: initialState.today_note_path,
			todayNoteContent: initialState.today_note_content,
		}));
	} catch (error) {
		console.error("Failed to initialize app:", error);
	}
};
