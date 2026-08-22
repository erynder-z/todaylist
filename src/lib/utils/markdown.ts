export const stripMarkdown = (markdown: string): string => {
	return (
		markdown
			// Images: ![alt](url) → alt
			.replace(/!\[([^\]]*)\]\([^)]+\)/g, "$1")
			// Links: [text](url) → text
			.replace(/\[([^\]]+)\]\([^)]+\)/g, "$1")
			// Headings
			.replace(/^#{1,6}\s+/gm, "")
			// Blockquotes
			.replace(/^>\s?/gm, "")
			// Unordered lists
			.replace(/^\s*[-*+]\s+/gm, "")
			// Ordered lists
			.replace(/^\s*\d+\.\s+/gm, "")
			// Task list markers
			.replace(/^\s*\[[ xX]\]\s+/gm, "")
			// Bold / italic
			.replace(/(\*\*|__)(.*?)\1/g, "$2")
			.replace(/(\*|_)(.*?)\1/g, "$2")
			// Strikethrough
			.replace(/~~(.*?)~~/g, "$1")
			// Inline code
			.replace(/`([^`]+)`/g, "$1")
			// Fenced code blocks
			.replace(/^```[\w-]*\n?/gm, "")
			// Trailing Markdown line-break
			.replace(/\\$/gm, "")
			.trim()
	);
};
