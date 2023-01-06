import type { LayoutKeysGroup } from "graphite-frontend-glue/editor_types";
import { PLATFORM } from "./platform";

export type Debouncer = ReturnType<typeof debouncer>;

export type DebouncerOptions = {
	debounceTime: number;
};

// eslint-disable-next-line @typescript-eslint/explicit-function-return-type
export function debouncer<T>(callFn: (value: T) => unknown, { debounceTime = 60 }: Partial<DebouncerOptions> = {}) {
	let currentValue: T | undefined;

	const emitValue = (): void => {
		if (currentValue === undefined) {
			throw new Error("Tried to emit undefined value from debouncer. This should never be possible");
		}
		const emittingValue = currentValue;
		currentValue = undefined;
		callFn(emittingValue);
	};

	const updateValue = (newValue: T): void => {
		if (currentValue !== undefined) {
			currentValue = newValue;
			return;
		}

		currentValue = newValue;
		setTimeout(emitValue, debounceTime);
	};

	return { updateValue };
}

class AssertionError extends Error {}

export function assert<T>(value: T, message?: string): NonNullable<T> {
	if (!value) {
		throw new AssertionError(message);
	}
	return value;
}

export function arraysEqual<T>(a: T[], b: T[]): boolean {
	return a.length === b.length && a.every((aValue, i) => aValue === b[i]);
}

// TODO: Apparently, Safari does not support the Keyboard.lock() API but does relax its authority over certain keyboard shortcuts in fullscreen mode, which we should take advantage of
const accelKey = PLATFORM === "Mac" ? "Command" : "Control";
const LOCK_REQUIRING_SHORTCUTS = [
	[accelKey, "KeyW"],
	[accelKey, "KeyN"],
	[accelKey, "Shift", "KeyN"],
	[accelKey, "KeyT"],
	[accelKey, "Shift", "KeyT"],
];

export function shortcutRequiresLock(shortcut: LayoutKeysGroup): boolean {
	const shortcutKeys = shortcut.map((keyWithLabel) => keyWithLabel.key);

	// If this shortcut matches any of the browser-reserved shortcuts
	return LOCK_REQUIRING_SHORTCUTS.some((lockKeyCombo) => arraysEqual(shortcutKeys, lockKeyCombo));
}
