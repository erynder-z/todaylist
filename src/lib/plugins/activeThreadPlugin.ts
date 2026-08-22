import { Plugin, PluginKey, type PluginView } from "@milkdown/kit/prose/state";
import {
	Decoration,
	DecorationSet,
	type EditorView,
} from "@milkdown/kit/prose/view";
import { $prose } from "@milkdown/kit/utils";

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
					index: -1 as number,
				};
			},

			apply(tr, value) {
				const meta = tr.getMeta(activeThreadPluginKey);

				if (meta !== undefined) {
					return {
						index: meta,
					};
				}

				return value;
			},
		},

		props: {
			decorations(state) {
				const activeIndex = activeThreadPluginKey.getState(state)?.index;

				if (
					activeIndex === undefined ||
					activeIndex === null ||
					activeIndex < 0
				)
					return DecorationSet.empty;

				const decorations: Decoration[] = [];
				let currentIndex = 0;

				state.doc.descendants((node, pos) => {
					if (node.type.name === "thread_marker") {
						if (currentIndex === activeIndex) {
							decorations.push(
								Decoration.node(pos, pos + node.nodeSize, {
									"data-active-thread": "true",
								}),
							);
						}
						currentIndex++;
					}
				});

				return DecorationSet.create(state.doc, decorations);
			},
		},

		view(): PluginView {
			let lastActiveIndex = -1;
			let scrollPending = false;

			return {
				update(view: EditorView) {
					const currentActiveIndex =
						activeThreadPluginKey.getState(view.state)?.index ?? -1;

					// Only scroll when the active thread changes and it's not invalid (-1)
					if (currentActiveIndex !== lastActiveIndex) {
						lastActiveIndex = currentActiveIndex;
						if (currentActiveIndex >= 0) {
							scrollPending = true;

							requestAnimationFrame(() => {
								if (!scrollPending) return;
								scrollPending = false;

								const { state } = view;
								let threadPos = -1;
								let currentIndex = 0;

								// Find the position of the active thread marker
								state.doc.descendants((node, pos) => {
									if (node.type.name === "thread_marker") {
										if (currentIndex === currentActiveIndex) {
											threadPos = pos;
											return false;
										}
										currentIndex++;
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
					}
				},
			};
		},
	});
});

/**
 * Updates the active thread displayed by the editor.
 */
export const setActiveThread = (view: EditorView, index: number) =>
	view.dispatch(view.state.tr.setMeta(activeThreadPluginKey, index));
