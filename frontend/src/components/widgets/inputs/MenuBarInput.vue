<template>
	<div class="menu-bar-input" data-menu-bar-input>
		<div class="entry-container" v-for="(entry, index) in entries" :key="index">
			<div
				@click="(e: MouseEvent) => clickEntry(entry, e)"
				@blur="(e: FocusEvent) => unFocusEntry(entry, e)"
				@keydown="(e: KeyboardEvent) => entry.ref?.keydown(e, false)"
				class="entry"
				:class="{ open: entry.ref?.isOpen }"
				tabindex="0"
				:data-floating-menu-spawner="entry.children && entry.children.length > 0 ? '' : 'no-hover-transfer'"
			>
				<IconLabel v-if="entry.icon" :icon="entry.icon" />
				<TextLabel v-if="entry.label">{{ entry.label }}</TextLabel>
			</div>
			<MenuList
				v-if="entry.children && entry.children.length > 0"
				:open="entry.ref?.open || false"
				:entries="entry.children || []"
				:direction="'Bottom'"
				:minWidth="240"
				:drawIcon="true"
				:ref="(ref: MenuListInstance): void => (ref && (entry.ref = ref), undefined)"
			/>
		</div>
	</div>
</template>

<style lang="scss">
.menu-bar-input {
	display: flex;

	.entry-container {
		display: flex;
		position: relative;

		.entry {
			display: flex;
			align-items: center;
			white-space: nowrap;
			padding: 0 8px;
			background: none;
			border: 0;
			margin: 0;

			svg {
				fill: var(--color-e-nearwhite);
			}

			&:hover,
			&.open {
				background: var(--color-6-lowergray);

				svg {
					fill: var(--color-f-white);
				}

				span {
					color: var(--color-f-white);
				}
			}
		}
	}
}
</style>

<script lang="ts">
import { defineComponent } from "vue";

import { platformIsMac } from "@/utility-functions/platform";
import { type KeyRaw, type LayoutKeysGroup, type MenuBarEntry, type MenuListEntry, UpdateMenuBarLayout } from "@/wasm-communication/messages";

import MenuList from "@/components/floating-menus/MenuList.vue";
import IconLabel from "@/components/widgets/labels/IconLabel.vue";
import TextLabel from "@/components/widgets/labels/TextLabel.vue";

// eslint-disable-next-line @typescript-eslint/no-unused-vars
type MenuListInstance = InstanceType<typeof MenuList>;

// TODO: Apparently, Safari does not support the Keyboard.lock() API but does relax its authority over certain keyboard shortcuts in fullscreen mode, which we should take advantage of
const accelKey = platformIsMac() ? "Command" : "Control";
const LOCK_REQUIRING_SHORTCUTS: KeyRaw[][] = [
	[accelKey, "KeyW"],
	[accelKey, "KeyN"],
	[accelKey, "Shift", "KeyN"],
	[accelKey, "KeyT"],
	[accelKey, "Shift", "KeyT"],
];

export default defineComponent({
	inject: ["editor"],
	mounted() {
		this.editor.subscriptions.subscribeJsMessage(UpdateMenuBarLayout, (updateMenuBarLayout) => {
			const arraysEqual = (a: KeyRaw[], b: KeyRaw[]): boolean => a.length === b.length && a.every((aValue, i) => aValue === b[i]);
			const shortcutRequiresLock = (shortcut: LayoutKeysGroup): boolean => {
				const shortcutKeys = shortcut.map((keyWithLabel) => keyWithLabel.key);

				// If this shortcut matches any of the browser-reserved shortcuts
				return LOCK_REQUIRING_SHORTCUTS.some((lockKeyCombo) => arraysEqual(shortcutKeys, lockKeyCombo));
			};

			const menuBarEntryToMenuListEntry = (entry: MenuBarEntry): MenuListEntry => ({
				// From `MenuEntryCommon`
				...entry,

				// Shared names with fields that need to be converted from the type used in `MenuBarEntry` to that of `MenuListEntry`
				action: (): void => this.editor.instance.updateLayout(updateMenuBarLayout.layoutTarget, entry.action.widgetId, undefined),
				children: entry.children ? entry.children.map((entries) => entries.map((entry) => menuBarEntryToMenuListEntry(entry))) : undefined,

				// New fields in `MenuListEntry`
				shortcutRequiresLock: entry.shortcut ? shortcutRequiresLock(entry.shortcut.keys) : undefined,
				value: undefined,
				disabled: undefined,
				font: undefined,
				ref: undefined,
			});

			this.entries = updateMenuBarLayout.layout.map(menuBarEntryToMenuListEntry);
		});
	},
	methods: {
		clickEntry(menuListEntry: MenuListEntry, e: MouseEvent) {
			// If there's no menu to open, trigger the action but don't try to open its non-existant children
			if (!menuListEntry.children || menuListEntry.children.length === 0) {
				if (menuListEntry.action && !menuListEntry.disabled) menuListEntry.action();

				return;
			}

			// Focus the target so that keyboard inputs are sent to the dropdown
			(e.target as HTMLElement | undefined)?.focus();

			if (menuListEntry.ref) menuListEntry.ref.isOpen = true;
			else throw new Error("The menu bar floating menu has no associated ref");
		},
		unFocusEntry(menuListEntry: MenuListEntry, e: FocusEvent) {
			const blurTarget = (e.target as HTMLElement | undefined)?.closest("[data-menu-bar-input]");
			const self: HTMLDivElement | undefined = this.$el;
			if (blurTarget !== self && menuListEntry.ref) menuListEntry.ref.isOpen = false;
		},
	},
	data() {
		return {
			entries: [] as MenuListEntry[],
			open: false,
		};
	},
	components: {
		IconLabel,
		MenuList,
		TextLabel,
	},
});
</script>
