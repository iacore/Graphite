import { browser } from "$app/environment";
import type { GraphiteEmitter } from "glue/emitter_type";
import type { JsEditorHandle } from "graphite-frontend-glue/editor";
import { writable, type Writable } from "svelte/store";
import { createEditor, createEmitter } from "graphite-frontend-glue/editor";
import { PLATFORM } from "./platform";

export type Document = {
	displayName: string;
	// todo
};

export type Portfolio = {
	activeDocumentIndex?: number;
	documents: Array<Document>;
};

export const pubsub: GraphiteEmitter = createEmitter();
export const editor: Writable<JsEditorHandle | undefined> = writable(undefined);
export const rootElement: Writable<HTMLElement | undefined> = writable(undefined);

export const maximized: Writable<boolean> = writable(false); // todo: patch
export const portfolio: Writable<Portfolio> = writable({
	activeDocumentIndex: undefined,
	documents: [],
}); // todo: patch

async function initEditor() {
	pubsub.on("UpdateMenuBarLayout", console.trace);
	const ed = await createEditor(pubsub);
	editor.set(ed);
	ed.initAfterFrontendReady(PLATFORM);
	// ed.initAfterFrontendReady
}

if (browser) {
	initEditor();
}
