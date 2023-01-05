import { sveltekit } from "@sveltejs/kit/vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import path from "path";
import { fileURLToPath } from "url";

/** @type {import('vite').UserConfig} */
const config = {
	resolve: {
		alias: {
			$icon: path.resolve(path.dirname(fileURLToPath(import.meta.url)), "icons"),
		},
	},
	plugins: [topLevelAwait(), wasm(), sveltekit()],
	server: {
		fs: {
			allow: ["glue", "icons"],
		},
	},
};

export default config;
