// for reference only
function menuBarEntryToMenuListEntry(layoutTarget: LayoutTarget, entry: MenuBarEntry): MenuListEntry {
	const ed = assert($editor);
	return {
		// From `MenuEntryCommon`
		...entry,

		// Shared names with fields that need to be converted from the type used in `MenuBarEntry` to that of `MenuListEntry`
		action: () => ed.updateLayout(layoutTarget, BigInt(entry.action.widgetId), undefined),

		w_children: entry.children.map((entries) => entries.map((entry) => menuBarEntryToMenuListEntry(layoutTarget, entry))),
	};
}

// todo: keyboard lock
// may not need it
