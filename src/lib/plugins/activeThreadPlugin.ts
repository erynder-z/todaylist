import { Plugin, PluginKey, type PluginView } from "@milkdown/prose/state";
import {
	Decoration,
	DecorationSet,
	type EditorView,
} from "@milkdown/prose/view";
import { $prose } from "@milkdown/utils";

/**
 * Plugin key used to store and access the active thread state.
 */
export const activeThreadPluginKey = new PluginKey("activeThread");

/**
 * ProseMirror plugin that highlights the currently active thread marker.
 */
export const activeThreadPlugin = $prose(() => {
	return new Plugin({
		key: activeThreadPluginKey,

		state: {
			init() {
				return {
					name: null as string | null,
				};
			},

			apply(tr, value) {
				const meta = tr.getMeta(activeThreadPluginKey);

				if (meta !== undefined) {
					return {
						name: meta,
					};
				}

				return value;
			},
		},

		props: {
			decorations(state) {
				const activeName = activeThreadPluginKey.getState(state)?.name;

				if (!activeName) {
					return DecorationSet.empty;
				}

				const decorations: Decoration[] = [];

				state.doc.descendants((node, pos) => {
					if (
						node.type.name === "thread_marker" &&
						node.textContent === activeName
					) {
						decorations.push(
							Decoration.node(pos, pos + node.nodeSize, {
								"data-active-thread": "true",
							}),
						);
					}
				});

				return DecorationSet.create(state.doc, decorations);
			},
		},

		view(): PluginView {
			let lastActiveName: string | null = null;
			let scrollPending = false;

			return {
				update(view: EditorView) {
					const currentActiveName = activeThreadPluginKey.getState(
						view.state,
					)?.name;

					// Only scroll when the active thread changes and it's not null
					if (
						currentActiveName !== lastActiveName &&
						currentActiveName !== null
					) {
						lastActiveName = currentActiveName;
						scrollPending = true;

						requestAnimationFrame(() => {
							if (!scrollPending) return;
							scrollPending = false;

							const { state } = view;
							let threadPos = -1;

							// Find the position of the active thread marker
							state.doc.descendants((node, pos) => {
								if (
									node.type.name === "thread_marker" &&
									node.textContent === currentActiveName
								) {
									threadPos = pos;
									return false; // Stop searching after finding the first match
								}
							});

							if (threadPos >= 0) {
								const dom = view.nodeDOM(threadPos);

								if (dom instanceof HTMLElement) {
									dom.scrollIntoView({
										behavior: "smooth",
										block: "start",
									});
								}
							}
						});
					}
				},
			};
		},
	});
});

/**
 * Updates the active thread displayed by the editor.
 */
export const setActiveThread = (view: EditorView, name: string | null) => {
	view.dispatch(view.state.tr.setMeta(activeThreadPluginKey, name));
};
