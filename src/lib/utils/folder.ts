import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { FormattedNote } from "$lib/types/notes";

export const selectFolder = async () => {
	try {
		const selected = await open({
			directory: true,
			multiple: false,
			title: "Select a folder",
		});
		return selected as string | null;
	} catch (error) {
		console.error("Error selecting folder:", error);
		return null;
	}
};

export const createNewNote = async () => {
	try {
		const filePath = (await invoke("create_new_note")) as string;
		return filePath;
	} catch (error) {
		console.error("Error creating new note:", error);
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

export const createTodaysNote = async () => {
	try {
		const filePath = (await invoke("create_todays_note")) as string;
		return filePath;
	} catch (error) {
		console.error("Error creating today's note:", error);
		return null;
	}
};

export const listNotes = async () => {
	try {
		const notes = (await invoke("list_notes")) as FormattedNote[];
		return notes;
	} catch (error) {
		console.error("Frontend: Error listing notes:", error);
		return null;
	}
};
