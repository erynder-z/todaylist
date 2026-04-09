import { untrack } from "svelte";
import type { NoteContentResponse } from "$lib/types/notes";
import { jumpToSection, saveNoteContent } from "$lib/utils/notes";

/**
 * Manages the state and logic for the Note Editor.
 * Handles markdown content, auto-saving, and section navigation.
 */
export class EditorStore {
	content = $state<string>("");
	noteContent = $state<NoteContentResponse | null>(null);
	notePath = $state<string | null>(null);
	private hasChanges = $state<boolean>(false);
	private autoSaveTimeout: ReturnType<typeof setTimeout> | null = null;

	// Cursor positioning after section jump
	cursorPosition = $state<number | null>(null);

	// Callback for section jumps
	onJump: (updated: NoteContentResponse) => void = () => {};

	// --- Initialization ---

	/**
	 * Synchronizes the store with the current note props.
	 */
	sync(noteContent: NoteContentResponse | null, notePath: string | null) {
		const pathChanged = untrack(() => this.notePath) !== notePath;
		const contentChanged = untrack(() => this.noteContent) !== noteContent;

		this.notePath = notePath;
		this.noteContent = noteContent;

		if (pathChanged || contentChanged) {
			this.content = noteContent?.content ?? "";
			this.hasChanges = false;
			this.cursorPosition = null;
		}
	}

	// --- Content Management ---

	/**
	 * Updates the markdown content and marks it as changed for auto-saving.
	 */
	updateContent(markdown: string) {
		this.content = markdown;
		this.hasChanges = true;
		this.scheduleAutoSave();
	}

	/**
	 * Jumps to a section by name via the backend and updates cursor position.
	 */
	async jumpToSection(name: string) {
		const updated = await jumpToSection(name);
		if (updated) {
			this.content = updated.content;

			// Calculate cursor position: end of section content (content-relative line index)
			const section = updated.sections.find((s) => s.name === name);
			if (section) {
				const lines = updated.content.split("\n");
				const endLine = section.endLine;
				// Calculate character position for cursor (start of endLine)
				let charPos = 0;
				for (let i = 0; i < endLine && i < lines.length; i++)
					charPos += lines[i].length + 1; // +1 for newline

				this.cursorPosition = charPos;
			}

			this.onJump(updated);
		}
	}

	// --- Persistence ---

	/**
	 * Persists the entire note content to the backend.
	 */
	async flush() {
		if (this.notePath && this.hasChanges) {
			await saveNoteContent(this.notePath, this.content);
			this.hasChanges = false;
		}
	}

	private scheduleAutoSave() {
		if (this.autoSaveTimeout) clearTimeout(this.autoSaveTimeout);

		this.autoSaveTimeout = setTimeout(() => {
			untrack(() => this.flush());
		}, 500);
	}

	// --- Keyboard Handling ---

	/**
	 * Handles keyboard events for the editor.
	 */
	async handleKeyDown(e: KeyboardEvent): Promise<boolean> {
		// Handle slash commands (e.g., "/SectionName")
		if (e.key === "Enter" && this.content.endsWith("/")) {
			const lines = this.content.split("\n");
			const lastLine = lines[lines.length - 1]?.trim() || "";

			if (lastLine.startsWith("/")) {
				const command = lastLine.slice(1).trim();
				if (command) {
					lines.pop(); // Remove the slash command line
					this.content = lines.join("\n");
					await this.jumpToSection(command);
					return true;
				}
			}
		}

		return false;
	}
}
