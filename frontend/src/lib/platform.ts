/* Platform specific code */

export type ApplicationPlatform = "Windows" | "Mac" | "Linux" | "Web";
export const PLATFORM: ApplicationPlatform = "Web";

import { maximized, rootElement } from "$lib/stores";
import { get } from "svelte/store";
import { assert } from "./utils";

export async function enterFullscreen(options?: FullscreenOptions) {
	await assert(get(rootElement), "The root component should be mounted by now").requestFullscreen(options);
	maximized.set(true);
}

export async function exitFullscreen() {
	await document.exitFullscreen();
	maximized.set(false);
}
