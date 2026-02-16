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
