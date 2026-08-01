import { type Editor, editorViewCtx, parserCtx } from "@milkdown/core";
import { Selection } from "@milkdown/prose/state";

/**
 * Service class for Milkdown editor operations.
 * Provides methods for navigating, updating, and focusing editor content.
 */
export class EditorService {
	constructor(private editor: Editor) {}

	/**
	 * Directs the editor to a specific thread by index.
	 * Finds the thread marker (!!!), moves the cursor to the end of that thread and scrolls to it.
	 */
	jumpToThreadByIndex(index: number) {
		this.editor.action((ctx) => {
			const view = ctx.get(editorViewCtx);
			const { doc } = view.state;

			const threadMarkers: { name: string; pos: number; endPos: number }[] = [];

			doc.descendants((node, pos) => {
				if (node.type.name === "thread_marker") {
					threadMarkers.push({
						name: node.textContent,
						pos,
						endPos: pos + node.nodeSize,
					});
				}
			});

			if (index < 0 || index >= threadMarkers.length) return;

			const thread = threadMarkers[index];
			const nextThread = threadMarkers[index + 1];

			// End of this thread's section:
			// either right before the next thread marker or end of document
			const cursorPos = nextThread ? nextThread.pos : doc.content.size;

			const resolvedPos = doc.resolve(cursorPos);
			const selection = Selection.near(resolvedPos, -1);

			// Move cursor
			view.dispatch(view.state.tr.setSelection(selection));

			// Scroll the thread marker itself
			requestAnimationFrame(() => {
				const dom = view.nodeDOM(thread.pos);

				if (dom instanceof HTMLElement) {
					dom.scrollIntoView({
						behavior: "smooth",
						block: "start",
					});
				}
			});

			view.focus();
		});
	}

	/**
	 * Updates the editor's content from a Markdown string and positions the cursor at the end.
	 */
	updateContent(markdown: string) {
		this.editor.action((ctx) => {
			const view = ctx.get(editorViewCtx);
			const parser = ctx.get(parserCtx);
			const doc = parser(markdown);
			if (!doc) return;

			let tr = view.state.tr.replaceWith(0, view.state.doc.content.size, doc);

			// Ensure trailing empty line for thread markers (Milkdown parser workaround)
			if (doc.lastChild?.type.name === "thread_marker") {
				const paragraph = view.state.schema.nodes.paragraph.create();
				tr = tr.insert(tr.doc.content.size, paragraph);
			}

			// Position cursor at end and focus
			const selection = Selection.atEnd(tr.doc);
			view.dispatch(tr.setSelection(selection).scrollIntoView());
			view.focus();
		});
	}

	/**
	 * Focuses the editor instance.
	 */
	focus() {
		this.editor.action((ctx) => ctx.get(editorViewCtx).focus());
	}

	/**
	 * Focuses the editor and moves selection to the end.
	 */
	focusEnd() {
		this.editor.action((ctx) => {
			const view = ctx.get(editorViewCtx);
			view.focus();
			const tr = view.state.tr;
			const selection = Selection.atEnd(tr.doc);
			view.dispatch(tr.setSelection(selection).scrollIntoView());
		});
	}
}
