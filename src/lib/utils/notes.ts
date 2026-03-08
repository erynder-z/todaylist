import { invoke } from "@tauri-apps/api/core";
import { MarkdownRenderCache } from "./renderCache";

const renderCache = new MarkdownRenderCache<string, string>(500);

export const readNoteContent = async (path: string) => {
	try {
		const content = (await invoke("read_note_content", { path })) as string;
		return content;
	} catch (error) {
		console.error(`Error reading note content from ${path}:`, error);
		return null;
	}
};

export const renderMarkdown = async (markdown: string) => {
	if (!markdown || !markdown.trim()) return "&nbsp;";

	// Return from cache if string is in cache
	const cached = renderCache.get(markdown);
	if (cached !== undefined) return cached;

	try {
		const html = (await invoke("render_markdown", { markdown })) as string;

		// Store the new result in cache
		renderCache.set(markdown, html);
		return html;
	} catch (error) {
		console.error("Error rendering markdown:", error);
		return markdown;
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

export const updateNoteLine = async (index: number, content: string) => {
	try {
		await invoke("update_note_line", { index, content });
		return true;
	} catch (error) {
		console.error("Error updating note line:", error);
		return false;
	}
};

export const insertNoteLine = async (index: number, content: string) => {
	try {
		await invoke("insert_note_line", { index, content });
		return true;
	} catch (error) {
		console.error("Error inserting note line:", error);
		return false;
	}
};

export const deleteNoteLine = async (index: number) => {
	try {
		await invoke("delete_note_line", { index });
		return true;
	} catch (error) {
		console.error("Error deleting note line:", error);
		return false;
	}
};
