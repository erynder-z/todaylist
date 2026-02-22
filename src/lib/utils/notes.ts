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
