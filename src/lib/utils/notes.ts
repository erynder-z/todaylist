import { invoke } from "@tauri-apps/api/core";

export const readNoteContent = async (path: string) => {
	try {
		const content = (await invoke("read_note_content", { path })) as string;
		return content;
	} catch (error) {
		console.error(`Error reading note content from ${path}:`, error);
		return null;
	}
};

export const saveNoteContent = async (path: string, content: string) => {
	try {
		await invoke("save_note_content", { path, content });
		return true;
	} catch (error) {
		console.error(`Error saving note content to ${path}:`, error);
		return false;
	}
};
