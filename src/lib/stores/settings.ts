import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";
import type { AppSettings } from "$lib/types/settings";
import { appState } from "./appState";

const createSettingsStore = () => {
	const { subscribe, set } = writable<AppSettings>({ notes_folder: "" });

	return {
		subscribe,
		set,

		async load() {
			try {
				const settings: AppSettings = await invoke("get_config");
				set(settings);
				return settings;
			} catch (error) {
				console.error("Error loading settings:", error);
				return { notes_folder: "" };
			}
		},

		async save(settings: AppSettings) {
			try {
				await invoke("set_notes_folder", { path: settings.notes_folder });
				set(settings);
				return true;
			} catch (error) {
				console.error("Error saving settings:", error);
				return false;
			}
		},

		async switchNotesFolder(path: string) {
			try {
				const newState: {
					notes_folder: string | null;
					today_note_path: string | null;
					today_note_content: string | null;
				} = await invoke("switch_notes_folder", { path });

				if (newState.notes_folder) {
					set({ notes_folder: newState.notes_folder });
				}

				appState.update((state) => ({
					...state,
					todayNotePath: newState.today_note_path,
					todayNoteContent: newState.today_note_content,
					activePopup: null,
				}));

				return true;
			} catch (error) {
				console.error("Error switching notes folder:", error);
				return false;
			}
		},
	};
};

export const settings = createSettingsStore();
