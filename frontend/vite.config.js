import { sveltekit } from '@sveltejs/kit/vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { ViteRsw } from 'vite-plugin-rsw';

/** @type {import('vite').UserConfig} */
const config = {
	plugins: [
	    ViteRsw(),
            //topLevelAwait(),
            //wasm(),
            sveltekit()
	]
};

export default config;
