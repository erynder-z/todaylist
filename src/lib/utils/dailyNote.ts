import { invoke } from "@tauri-apps/api/core";

/**
 * Retrieves the absolute path to the current daily note file from the backend.
 */
export const getTodayNotePath = async () => {
	try {
		const path = (await invoke("get_today_note_path")) as string;
		return path;
	} catch (error) {
		console.error("Error getting today's note path:", error);
		return null;
	}
};

/**
 * Checks if the current daily note file already exists in the configured folder.
 */
export const checkTodaysNoteExists = async () => {
	try {
		const exists = (await invoke("check_todays_note_exists")) as boolean;
		return exists;
	} catch (error) {
		console.error("Error checking today's note:", error);
		return false;
	}
};

/**
 * Creates a new daily note file at the specified path if it doesn't already exist.
 */
export const createTodaysNote = async (path: string) => {
	try {
		await invoke("create_todays_note", { path });
	} catch (error) {
		console.error("Error creating today's note:", error);
	}
};
