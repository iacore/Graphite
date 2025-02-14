<script lang="ts">
	import { createEventDispatcher } from "svelte";

	import { type MenuListEntry } from "@/wasm-communication/messages";

	import FloatingMenu, { type MenuDirection } from "@/components/layout/FloatingMenu.svelte";
	import LayoutCol from "@/components/layout/LayoutCol.svelte";
	import LayoutRow from "@/components/layout/LayoutRow.svelte";
	import IconLabel from "@/components/widgets/labels/IconLabel.svelte";
	import Separator from "@/components/widgets/labels/Separator.svelte";
	import TextLabel from "@/components/widgets/labels/TextLabel.svelte";
	import UserInputLabel from "@/components/widgets/labels/UserInputLabel.svelte";

	let self: FloatingMenu;
	let scroller: LayoutCol;

	// emits: ["update:open", "update:activeEntry", "naturalWidth"],
	const dispatch = createEventDispatcher<{ open: boolean; activeEntry: MenuListEntry }>();

	export let entries: MenuListEntry[][];
	export let activeEntry: MenuListEntry | undefined = undefined;
	export let open: boolean;
	export let direction: MenuDirection = "Bottom";
	export let minWidth = 0;
	export let drawIcon = false;
	export let interactive = false;
	export let scrollableY = false;
	export let virtualScrollingEntryHeight = 0;
	export let tooltip: string | undefined = undefined;

	let isOpen = open;
	let highlighted = activeEntry as MenuListEntry | undefined;
	let virtualScrollingEntriesStart = 0;

	// Called only when `open` is changed from outside this component (with v-model)
	$: watchOpen(open);
	$: dispatch("open", isOpen);
	$: watchEntries(entries, self);
	$: watchDrawIcon(drawIcon, self);
	$: virtualScrollingTotalHeight = entries.length === 0 ? 0 : entries[0].length * virtualScrollingEntryHeight;
	$: virtualScrollingStartIndex = Math.floor(virtualScrollingEntriesStart / virtualScrollingEntryHeight) || 0;
	$: virtualScrollingEndIndex = entries.length === 0 ? 0 : Math.min(entries[0].length, virtualScrollingStartIndex + 1 + 400 / virtualScrollingEntryHeight);

	function watchOpen(open: boolean) {
		isOpen = open;
		highlighted = activeEntry;
	}

	// TODO: Svelte: fix infinite loop and reenable
	function watchEntries(_: MenuListEntry[][], floatingMenu: FloatingMenu) {
		// floatingMenu?.div().measureAndEmitNaturalWidth();
	}

	// TODO: Svelte: fix infinite loop and reenable
	function watchDrawIcon(_: boolean, floatingMenu: FloatingMenu) {
		// floatingMenu?.div().measureAndEmitNaturalWidth();
	}

	function onScroll(e: Event) {
		if (!virtualScrollingEntryHeight) return;
		virtualScrollingEntriesStart = (e.target as HTMLElement)?.scrollTop || 0;
	}

	function onEntryClick(menuListEntry: MenuListEntry): void {
		// Call the action if available
		if (menuListEntry.action) menuListEntry.action();

		// Emit the clicked entry as the new active entry
		dispatch("activeEntry", menuListEntry);

		// Close the containing menu
		if (menuListEntry.ref) menuListEntry.ref.isOpen = false;
		dispatch("open", false);
		isOpen = false; // TODO: This is a hack for MenuBarInput submenus, remove it when we get rid of using `ref`
	}

	function onEntryPointerEnter(menuListEntry: MenuListEntry): void {
		if (!menuListEntry.children?.length) return;

		if (menuListEntry.ref) menuListEntry.ref.isOpen = true;
		else dispatch("open", true);
	}

	function onEntryPointerLeave(menuListEntry: MenuListEntry): void {
		if (!menuListEntry.children?.length) return;

		if (menuListEntry.ref) menuListEntry.ref.isOpen = false;
		else dispatch("open", false);
	}

	function isEntryOpen(menuListEntry: MenuListEntry): boolean {
		if (!menuListEntry.children?.length) return false;

		return open;
	}

	/// Handles keyboard navigation for the menu. Returns if the entire menu stack should be dismissed
	export function keydown(e: KeyboardEvent, submenu: boolean): boolean {
		// Interactive menus should keep the active entry the same as the highlighted one
		if (interactive) highlighted = activeEntry;

		const menuOpen = isOpen;
		const flatEntries = entries.flat().filter((entry) => !entry.disabled);
		const openChild = flatEntries.findIndex((entry) => entry.children?.length && entry.ref?.isOpen);

		const openSubmenu = (highlighted: MenuListEntry): void => {
			if (highlighted.ref && highlighted.children?.length) {
				highlighted.ref.isOpen = true;

				// Highlight first item
				highlighted.ref.setHighlighted(highlighted.children[0][0]);
			}
		};

		if (!menuOpen && (e.key === " " || e.key === "Enter")) {
			// Allow opening menu with space or enter
			isOpen = true;
			highlighted = activeEntry;
		} else if (menuOpen && openChild >= 0) {
			// Redirect the keyboard navigation to a submenu if one is open
			const shouldCloseStack = flatEntries[openChild].ref?.keydown(e, true);

			// Highlight the menu item in the parent list that corresponds with the open submenu
			if (e.key !== "Escape" && highlighted) setHighlighted(flatEntries[openChild]);

			// Handle the child closing the entire menu stack
			if (shouldCloseStack) {
				isOpen = false;
				return true;
			}
		} else if ((menuOpen || interactive) && (e.key === "ArrowUp" || e.key === "ArrowDown")) {
			// Navigate to the next and previous entries with arrow keys

			let newIndex = e.key === "ArrowUp" ? flatEntries.length - 1 : 0;
			if (highlighted) {
				const index = highlighted ? flatEntries.map((entry) => entry.label).indexOf(highlighted.label) : 0;
				newIndex = index + (e.key === "ArrowUp" ? -1 : 1);

				// Interactive dropdowns should lock at the end whereas other dropdowns should loop
				if (interactive) newIndex = Math.min(flatEntries.length - 1, Math.max(0, newIndex));
				else newIndex = (newIndex + flatEntries.length) % flatEntries.length;
			}

			const newEntry = flatEntries[newIndex];
			setHighlighted(newEntry);
		} else if (menuOpen && e.key === "Escape") {
			// Close menu with escape key
			isOpen = false;

			// Reset active to before open
			setHighlighted(activeEntry);
		} else if (menuOpen && highlighted && e.key === "Enter") {
			// Handle clicking on an option if enter is pressed
			if (!highlighted.children?.length) onEntryClick(highlighted);
			else openSubmenu(highlighted);

			// Stop the event from triggering a press on a new dialog
			e.preventDefault();

			// Enter should close the entire menu stack
			return true;
		} else if (menuOpen && highlighted && e.key === "ArrowRight") {
			// Right arrow opens a submenu
			openSubmenu(highlighted);
		} else if (menuOpen && e.key === "ArrowLeft") {
			// Left arrow closes a submenu
			if (submenu) isOpen = false;
		}

		// By default, keep the menu stack open
		return false;
	}

	export function setHighlighted(newHighlight: MenuListEntry | undefined) {
		highlighted = newHighlight;
		// Interactive menus should keep the active entry the same as the highlighted one
		if (interactive && newHighlight?.value !== activeEntry?.value && newHighlight) dispatch("activeEntry", newHighlight);
	}

	// TODO: Svelte: Re-enable the `export` prefix
	export function scrollViewTo(distanceDown: number): void {
		scroller.div().scrollTo(0, distanceDown);
	}

	export function menuIsOpen(): boolean {
		return open;
	}
</script>

<FloatingMenu
	class="menu-list"
	open={isOpen}
	on:open={({ detail }) => (isOpen = detail)}
	on:naturalWidth
	type="Dropdown"
	windowEdgeMargin={0}
	escapeCloses={false}
	{direction}
	{minWidth}
	scrollableY={scrollableY && virtualScrollingEntryHeight === 0}
	bind:this={self}
>
	<!-- If we put the scrollableY on the layoutcol for non-font dropdowns then for some reason it always creates a tiny scrollbar.
	However when we are using the virtual scrolling then we need the layoutcol to be scrolling so we can bind the events without using $refs. -->
	<LayoutCol
		bind:this={scroller}
		scrollableY={scrollableY && virtualScrollingEntryHeight !== 0}
		on:scroll={onScroll}
		styles={{ "min-width": virtualScrollingEntryHeight ? `${minWidth}px` : `inherit` }}
	>
		{#if virtualScrollingEntryHeight}
			<LayoutRow class="scroll-spacer" styles={{ height: `${virtualScrollingStartIndex * virtualScrollingEntryHeight}px` }} />
		{/if}
		{#each entries as section, sectionIndex (sectionIndex)}
			{#if sectionIndex > 0}
				<Separator type="List" direction="Vertical" />
			{/if}
			{#each virtualScrollingEntryHeight ? section.slice(virtualScrollingStartIndex, virtualScrollingEndIndex) : section as entry, entryIndex (entryIndex + (virtualScrollingEntryHeight ? virtualScrollingStartIndex : 0))}
				<LayoutRow
					class="row"
					classes={{ open: isEntryOpen(entry), active: entry.label === highlighted?.label, disabled: Boolean(entry.disabled) }}
					styles={{ height: virtualScrollingEntryHeight || "20px" }}
					{tooltip}
					on:click={() => !entry.disabled && onEntryClick(entry)}
					on:pointerenter={() => !entry.disabled && onEntryPointerEnter(entry)}
					on:pointerleave={() => !entry.disabled && onEntryPointerLeave(entry)}
				>
					{#if entry.icon && drawIcon}
						<IconLabel icon={entry.icon} class="entry-icon" />
					{:else if drawIcon}
						<div class="no-icon" />
					{/if}

					{#if entry.font}
						<link rel="stylesheet" href={entry.font?.toString()} />
					{/if}

					<TextLabel class="entry-label" styles={{ "font-family": `${!entry.font ? "inherit" : entry.value}` }}>{entry.label}</TextLabel>

					{#if entry.shortcut?.keys.length}
						<UserInputLabel keysWithLabelsGroups={[entry.shortcut.keys]} requiresLock={entry.shortcutRequiresLock} />
					{/if}

					{#if entry.children?.length}
						<div class="submenu-arrow" />
					{:else}
						<div class="no-submenu-arrow" />
					{/if}

					{#if entry.children}
						<svelte:self on:naturalWidth open={entry.ref?.menuIsOpen() || false} direction="TopRight" entries={entry.children} {minWidth} {drawIcon} {scrollableY} bind:this={entry.ref} />
					{/if}
				</LayoutRow>
			{/each}
		{/each}
		{#if virtualScrollingEntryHeight}
			<LayoutRow class="scroll-spacer" styles={{ height: `${virtualScrollingTotalHeight - virtualScrollingEndIndex * virtualScrollingEntryHeight}px` }} />
		{/if}
	</LayoutCol>
</FloatingMenu>

<style lang="scss" global>
	.menu-list {
		.floating-menu-container .floating-menu-content {
			padding: 4px 0;

			.separator div {
				background: var(--color-4-dimgray);
			}

			.scroll-spacer {
				flex: 0 0 auto;
			}

			.row {
				height: 20px;
				align-items: center;
				white-space: nowrap;
				position: relative;
				flex: 0 0 auto;

				& > * {
					flex: 0 0 auto;
				}

				.entry-icon svg {
					fill: var(--color-e-nearwhite);
				}

				.no-icon {
					width: 16px;
				}

				.entry-label {
					flex: 1 1 100%;
					margin-left: 8px;
				}

				.entry-icon,
				.no-icon {
					margin: 0 4px;

					& + .entry-label {
						margin-left: 0;
					}
				}

				.user-input-label {
					margin-left: 16px;
				}

				.submenu-arrow {
					width: 0;
					height: 0;
					border-style: solid;
					border-width: 3px 0 3px 6px;
					border-color: transparent transparent transparent var(--color-e-nearwhite);
				}

				.no-submenu-arrow {
					width: 6px;
				}

				.submenu-arrow,
				.no-submenu-arrow {
					margin-left: 6px;
					margin-right: 4px;
				}

				&:hover,
				&.open {
					background: var(--color-6-lowergray);
					color: var(--color-f-white);

					.entry-icon svg {
						fill: var(--color-f-white);
					}
				}

				&.active {
					background: var(--color-e-nearwhite);
					color: var(--color-2-mildblack);

					.entry-icon svg {
						fill: var(--color-2-mildblack);
					}
				}

				&.disabled {
					color: var(--color-8-uppergray);

					&:hover {
						background: none;
					}

					svg {
						fill: var(--color-8-uppergray);
					}
				}
			}
		}
	}
</style>
