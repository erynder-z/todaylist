import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";
import type { AppSettings } from "$lib/types/settings";

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
	};
};

export const settings = createSettingsStore();
