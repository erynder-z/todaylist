/**
 * Store for managing generic list navigation via keyboard.
 * Handles ArrowUp, ArrowDown, and Enter selection.
 */
export class ListNavigator {
	/** Current index of the highlighted item in the list. */
	index = $state(-1);

	constructor(
		private getCount: () => number,
		private onSelect: (index: number) => void,
		private options: { wrap?: boolean } = { wrap: true },
	) {}

	/**
	 * Handles keyboard events for navigation and selection.
	 * Returns true if the key event was handled.
	 */
	handleKey(e: KeyboardEvent): boolean {
		const count = this.getCount();
		if (count === 0) return false;

		switch (e.key) {
			case "ArrowDown":
				e.preventDefault();
				if (this.options.wrap) {
					this.index = (this.index + 1) % count;
				} else {
					this.index = Math.min(this.index + 1, count - 1);
				}
				return true;

			case "ArrowUp":
				e.preventDefault();
				if (this.options.wrap) {
					this.index = (this.index - 1 + count) % count;
				} else {
					this.index = Math.max(this.index - 1, 0);
				}
				return true;

			case "Enter":
				if (this.index >= 0 && this.index < count) {
					e.preventDefault();
					this.onSelect(this.index);
					return true;
				}
				break;
		}

		return false;
	}

	/**
	 * Resets the highlighted index back to -1.
	 */
	reset() {
		this.index = -1;
	}
}
