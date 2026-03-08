/**
 * A simple in-memory cache with a maximum size limit to prevent memory leaks.
 * Used primarily for storing rendered Markdown HTML to avoid redundant backend calls.
 */
export class MarkdownRenderCache<K, V> {
	private cache = new Map<K, V>();
	private maxSize: number;

	constructor(maxSize = 500) {
		this.maxSize = maxSize;
	}

	/**
	 * Retrieves a value from the cache by its key.
	 */
	get(key: K): V | undefined {
		return this.cache.get(key);
	}

	/**
	 * Adds a value to the cache. If the cache is full, it removes the oldest entry.
	 */
	set(key: K, value: V): void {
		if (!this.cache.has(key) && this.cache.size >= this.maxSize) {
			const firstKey = this.cache.keys().next().value;
			if (firstKey !== undefined) {
				this.cache.delete(firstKey);
			}
		}
		this.cache.set(key, value);
	}

	/**
	 * Checks if a key exists in the cache.
	 */
	has(key: K): boolean {
		return this.cache.has(key);
	}

	/**
	 * Clears all entries from the cache.
	 */
	clear(): void {
		this.cache.clear();
	}
}
