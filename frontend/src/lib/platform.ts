/* Platform specific code */

export type ApplicationPlatform = "Windows" | "Mac" | "Linux" | "Web";
export const PLATFORM: ApplicationPlatform = "Web";

import { maximized } from "$lib/stores";

export function toggleFullscreen() {
	maximized.set(true);
}
