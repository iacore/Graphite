import { browser } from "$app/environment";
import type { LayoutTarget, MenuBarEntry } from "glue/editor_types";
import type { GraphiteEmitter } from "glue/emitter_type";
import type { JsEditorHandle } from "graphite-frontend-glue/editor";
import { createEditor, createEmitter } from "graphite-frontend-glue/editor";
import { writable, type Writable } from "svelte/store";
import { PLATFORM } from "./platform";

export type Document = {
	displayName: string;
	// todo
};

export type Portfolio = {
	activeDocumentIndex?: number;
	documents: Array<Document>;
};

/** Don't subscribe to the updates. Use the stores */
export const pubsub: GraphiteEmitter = createEmitter();
export const editor: Writable<JsEditorHandle | undefined> = writable(undefined);
export const rootElement: Writable<HTMLElement | undefined> = writable(undefined);
export const menuBarLayout: Writable<Partial<Record<LayoutTarget, Array<MenuBarEntry>>>> = writable({});

pubsub.on("UpdateMenuBarLayout", ({ layoutTarget, layout }) => {
	menuBarLayout.update((a) => ((a[layoutTarget] = layout), a));
});

export const maximized: Writable<boolean> = writable(false); // todo: patch
export const portfolio: Writable<Portfolio> = writable({
	activeDocumentIndex: undefined,
	documents: [],
}); // todo: patch

async function initEditor() {
	const ed = await createEditor(pubsub);
	editor.set(ed);
	ed.initAfterFrontendReady(PLATFORM);
}

if (browser) {
	initEditor();
}
