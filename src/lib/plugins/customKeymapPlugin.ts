import { keymap } from "@milkdown/kit/prose/keymap";
import { Selection } from "@milkdown/kit/prose/state";
import { $prose } from "@milkdown/kit/utils";

/**
 * Custom keymap for the note editor.
 *
 * Disables Milkdown's built-in heading/mark shortcuts so they don't
 * interfere with the app's own shortcut handling, and adds a Backspace
 * handler for deleting an empty thread marker that is the first block
 * in the document.
 *
 * ProseMirror's default joinBackward can't remove a thread marker in that
 * position (there is no preceding block to join with, and the doc can't be
 * emptied), so without this the last remaining thread marker becomes
 * impossible to delete. Only this edge case is handled — everything else
 * falls through to the default Backspace behavior, preserving the existing
 * multi-thread delete behavior.
 */
export const customKeymapPlugin = $prose(() =>
	keymap({
		"Mod-Alt-1": () => true,
		"Mod-Alt-2": () => true,
		"Mod-Alt-3": () => true,
		"Mod-Alt-4": () => true,
		"Mod-Alt-5": () => true,
		"Mod-Alt-6": () => true,
		"Mod-b": () => true,
		"Mod-i": () => true,
		"Mod-e": () => true,
		"Mod-x`": () => true,
		Backspace: (state, dispatch) => {
			const { selection, doc } = state;
			if (!selection.empty) return false;
			const resolved = selection.$from;
			if (resolved.parent.type.name !== "thread_marker") return false;
			if (resolved.parentOffset !== 0 || resolved.parent.textContent.length > 0)
				return false;
			// Only when it's the first top-level block; otherwise let the
			// default handler join/lift as usual.
			if (resolved.index(0) !== 0) return false;

			const start = resolved.before(resolved.depth);
			const end = start + resolved.parent.nodeSize;
			const tr = state.tr;

			if (doc.childCount === 1) {
				// Removing the only block would empty the doc — swap in a paragraph.
				const paragraph = state.schema.nodes.paragraph.create();
				tr.replaceWith(start, end, paragraph);
				tr.setSelection(Selection.near(tr.doc.resolve(start)));
			} else {
				tr.delete(start, end);
				tr.setSelection(Selection.near(tr.doc.resolve(Math.max(0, start))));
			}

			if (dispatch) dispatch(tr.scrollIntoView());
			return true;
		},
	}),
);
