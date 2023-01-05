import { browser } from "$app/environment";
import type { GraphiteEmitter } from "glue/emitter_type";
import type { JsEditorHandle } from "graphite-frontend-glue/editor";
import { writable, type Writable } from "svelte/store";

export type Document = {
	displayName: string;
	// todo
};

export type Portfolio = {
	activeDocumentIndex?: number;
	documents: Array<Document>;
};

export const editor: Writable<JsEditorHandle | undefined> = writable(undefined);
export const pubsub: Writable<GraphiteEmitter | undefined> = writable(undefined);

export const maximized: Writable<boolean> = writable(false); // todo: patch
export const portfolio: Writable<Portfolio> = writable({
	activeDocumentIndex: undefined,
	documents: [],
}); // todo: patch

export async function initEditor() {
	const { editor: _editor, editor_pubsub } = await import("graphite-frontend-glue/editor");
	editor.set(_editor);
	pubsub.set(editor_pubsub);
}

if (browser) {
	initEditor();
}
