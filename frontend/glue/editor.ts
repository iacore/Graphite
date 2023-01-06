import { invoke } from "@tauri-apps/api";

import type { JsEditorHandle } from "graphite-wasm";
import { createNanoEvents } from "nanoevents";
export type { JsEditorHandle } from "graphite-wasm";

import type { GraphiteEmitter } from "./emitter_type";
export type { GraphiteEmitter } from "./emitter_type";

// todo: remove this
/**
 * @deprecated
 */
let lastEditor: JsEditorHandle | undefined = undefined;

export const createEmitter = createNanoEvents as () => GraphiteEmitter;

/* init wasm module */
export async function createEditor(pubsub: GraphiteEmitter): Promise<JsEditorHandle> {
	const { setRandomSeed, JsEditorHandle } = await import("graphite-wasm");
	const randomSeedFloat = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
	const randomSeed = BigInt(randomSeedFloat);
	setRandomSeed(randomSeed);
	let editor = (lastEditor = new JsEditorHandle((messageType: string, messageData: any): void => {
		pubsub.emit(messageType as any, (messageData ?? {})[messageType]);
	}));
	return editor;
}

export async function updateImage(path: BigUint64Array, mime: string, imageData: Uint8Array, documentId: bigint): Promise<void> {
	const blob = new Blob([imageData], { type: mime });

	const blobURL = URL.createObjectURL(blob);

	// Pre-decode the image so it is ready to be drawn instantly once it's placed into the viewport SVG
	const image = new Image();
	image.src = blobURL;
	await image.decode();

	lastEditor?.setImageBlobURL(documentId, path, blobURL, image.naturalWidth, image.naturalHeight);
	// todo: return this
	// return {documentId, path, blobURL, width: image.naturalWidth, height: image.naturalHeight}
}

export async function fetchImage(path: BigUint64Array, mime: string, documentId: bigint, url: string): Promise<void> {
	const data = await fetch(url);
	const blob = await data.blob();

	const blobURL = URL.createObjectURL(blob);

	// Pre-decode the image so it is ready to be drawn instantly once it's placed into the viewport SVG
	const image = new Image();
	image.src = blobURL;
	await image.decode();

	lastEditor?.setImageBlobURL(documentId, path, blobURL, image.naturalWidth, image.naturalHeight);

	// todo: return this
	// return {documentId, path, blobURL, width: image.naturalWidth, height: image.naturalHeight}
}

export async function dispatchTauri(message: unknown): Promise<void> {
	try {
		const response = await invoke("handle_message", { message });
		lastEditor?.tauriResponse(response);
		// todo: return this
		// return response
	} catch {
		console.error("Failed to dispatch Tauri message");
	}
}
