import { invoke } from "@tauri-apps/api/core";

export const getTodayNotePath = async () => {
	try {
		const path = (await invoke("get_today_note_path")) as string;
		return path;
	} catch (error) {
		console.error("Error getting today's note path:", error);
		return null;
	}
};

export const checkTodaysNoteExists = async () => {
	try {
		const exists = (await invoke("check_todays_note_exists")) as boolean;
		return exists;
	} catch (error) {
		console.error("Error checking today's note:", error);
		return false;
	}
};

export const createTodaysNote = async (path: string) => {
	try {
		await invoke("create_todays_note", { path });
	} catch (error) {
		console.error("Error creating today's note:", error);
	}
};
