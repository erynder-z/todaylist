import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

export async function selectFolder(): Promise<string | null> {
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
}

export async function createNewNote(): Promise<string | null> {
	try {
		const filePath = (await invoke("create_new_note")) as string;
		return filePath;
	} catch (error) {
		console.error("Error creating new note:", error);
		return null;
	}
}
