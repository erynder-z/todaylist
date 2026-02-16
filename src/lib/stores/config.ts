import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";
import type { AppConfig } from "$lib/types/app";

const createConfigStore = () => {
	const { subscribe, set } = writable<AppConfig>({ notes_folder: "" });

	return {
		subscribe,

		async load() {
			try {
				const config: AppConfig = await invoke("get_config");
				set(config);
				return config;
			} catch (error) {
				console.error("Error loading config:", error);
				return { notes_folder: "" };
			}
		},

		async save(config: AppConfig) {
			try {
				await invoke("set_notes_folder", { path: config.notes_folder });
				set(config);
				return true;
			} catch (error) {
				console.error("Error saving config:", error);
				return false;
			}
		},
	};
};

export const config = createConfigStore();
