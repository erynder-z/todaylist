import { get } from "svelte/store";
import { appState, config } from "$lib";
import {
	checkTodaysNoteExists,
	createTodaysNote,
	getTodayNotePath,
} from "$lib/utils/dailyNote";
import { readNoteContent } from "$lib/utils/notes";

export const initializeApp = async () => {
	await config.load();

	const currentConfig = get(config);
	if (currentConfig.notes_folder) {
		const todayNotePath = await getTodayNotePath();

		if (todayNotePath) {
			const exists = await checkTodaysNoteExists();
			if (!exists) await createTodaysNote(todayNotePath);

			const content = await readNoteContent(todayNotePath);
			appState.update((state) => ({
				...state,
				todayNotePath: todayNotePath,
				todayNoteContent: content,
			}));
		}
	}
};
