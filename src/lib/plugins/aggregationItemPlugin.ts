import { $nodeSchema, $remark } from "@milkdown/kit/utils";
import type { Node as MdastNode } from "unist";

/**
 * Remark plugin that transforms aggregation items into custom AST nodes.
 */
export const remarkAggregationItemPlugin = $remark(
	"remarkAggregationItem",
	() => () => (tree: MdastNode) => {
		const root = tree as MdastNode & { children: MdastNode[] };
		if (!root.children) return;

		const newChildren: MdastNode[] = [];
		let i = 0;

		while (i < root.children.length) {
			const node = root.children[i];

			if (node.type === "heading") {
				const heading = node as unknown as MdastNode & {
					depth: number;
					children: MdastNode[];
				};

				if (heading.depth === 2 && heading.children?.length > 0) {
					const firstChild = heading.children[0] as unknown as MdastNode & {
						type: string;
						url?: string;
						children?: MdastNode[];
					};
					const isAggregationHeader =
						firstChild?.type === "link" &&
						firstChild.url?.includes("todaynote.internal/open/");

					if (isAggregationHeader) {
						const contentNodes: MdastNode[] = [];
						let j = i + 1;

						while (j < root.children.length) {
							const nextNode = root.children[j];
							let isNextAggregationHeader = false;

							if (nextNode.type === "heading") {
								const nextHeading = nextNode as unknown as MdastNode & {
									depth: number;
									children: MdastNode[];
								};
								if (
									nextHeading.depth === 2 &&
									nextHeading.children?.length > 0
								) {
									const nextFirstChild = nextHeading
										.children[0] as unknown as MdastNode & {
										type: string;
										url?: string;
									};
									isNextAggregationHeader =
										(nextFirstChild?.type === "link" &&
											nextFirstChild.url?.includes(
												"todaynote.internal/open/",
											)) ||
										false;
								}
							}

							if (isNextAggregationHeader) break;
							contentNodes.push(nextNode);
							j++;
						}

						const linkNode = firstChild as unknown as MdastNode & {
							children: Array<{ value: string }>;
							url: string;
						};
						const dateText = linkNode.children?.[0]?.value || "";
						const url = linkNode.url || "";
						const urlParts = url.split("/");
						const filename = decodeURIComponent(
							urlParts[urlParts.length - 2] || "",
						);
						const threadId = decodeURIComponent(
							urlParts[urlParts.length - 1] || "",
						);

						const aggregationItem: MdastNode = {
							type: "aggregationItem",
							data: { date: dateText, filename, threadId },
							children: [node, ...contentNodes],
							position: node.position,
						} as unknown as MdastNode;

						newChildren.push(aggregationItem);
						i = j;
						continue;
					}
				}
			}

			newChildren.push(node);
			i++;
		}

		root.children = newChildren;
	},
);

/**
 * ProseMirror node schema for aggregation items.
 */
export const aggregationItemSchema = $nodeSchema("aggregation_item", () => ({
	group: "block",
	content: "block+",
	marks: "",
	defining: true,

	attrs: {
		date: { default: "" },
		filename: { default: "" },
		threadId: { default: "" },
	},

	parseDOM: [
		{
			tag: "div.aggregation-list-item",
			getAttrs: (dom) => ({
				date: (dom as HTMLElement).dataset.date || "",
				filename: (dom as HTMLElement).dataset.filename || "",
				threadId: (dom as HTMLElement).dataset.threadId || "",
			}),
		},
	],

	toDOM: (node) => [
		"div",
		{
			class: "aggregation-list-item",
			"data-date": node.attrs.date,
			"data-filename": node.attrs.filename,
			"data-thread-id": node.attrs.threadId,
		},
		0,
	],

	parseMarkdown: {
		match: (node) => node.type === "aggregationItem",
		runner: (state, node, type) => {
			const data = (
				node as unknown as {
					data?: { date?: string; filename?: string; threadId?: string };
				}
			).data;
			state.openNode(type, {
				date: data?.date || "",
				filename: data?.filename || "",
				threadId: data?.threadId || "",
			});
			const children = node.children || [];
			state.next(children);
			state.closeNode();
		},
	},

	toMarkdown: {
		match: (node) => node.type.name === "aggregation_item",
		runner: (state, node) => {
			state.addNode("paragraph", undefined, node.textContent);
		},
	},
}));

export const aggregationItemPlugin = [
	remarkAggregationItemPlugin,
	aggregationItemSchema,
].flat();
