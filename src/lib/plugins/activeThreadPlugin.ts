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
					shouldScroll: false,
				};
			},

			apply(tr, value) {
				const meta = tr.getMeta(activeThreadPluginKey);

				if (meta !== undefined) {
					return {
						index: meta,
						shouldScroll: meta >= 0,
					};
				}

				return {
					...value,
					shouldScroll: false,
				};
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
			let scrollPending = false;

			return {
				update(view: EditorView) {
					const pluginState = activeThreadPluginKey.getState(view.state);
					const currentActiveIndex = pluginState?.index ?? -1;
					const shouldScroll = pluginState?.shouldScroll ?? false;

					if (shouldScroll && currentActiveIndex >= 0) {
						scrollPending = true;

						requestAnimationFrame(() => {
							if (!scrollPending) return;
							scrollPending = false;

							const { state } = view;
							const threadPositions: number[] = [];

							// Find the positions of all thread markers
							state.doc.descendants((node, pos) => {
								if (node.type.name === "thread_marker")
									threadPositions.push(pos);
							});

							if (
								currentActiveIndex >= 0 &&
								currentActiveIndex < threadPositions.length
							) {
								const threadPos = threadPositions[currentActiveIndex];
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
export const setActiveThread = (view: EditorView, index: number) =>
	view.dispatch(view.state.tr.setMeta(activeThreadPluginKey, index));
