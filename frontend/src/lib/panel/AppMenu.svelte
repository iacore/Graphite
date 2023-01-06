<script lang="ts">
import MenuList from "$lib/popup/MenuList.svelte"
import IconLabel from "../widget/labels/IconLabel.svelte"

import type {
  LayoutKeysGroup,
  LayoutTarget,
  MenuBarEntry,
} from "graphite-frontend-glue/editor_types"
import { onDestroy, onMount } from "svelte"
import { editor, menuBarLayout } from "$lib/stores"
import { PLATFORM } from "$lib/platform"
import { arraysEqual, assert } from "$lib/utils"

let entries: MenuListEntry[] = []
// let open: boolean = false

$: {
  entries = []
  for (const [key, value] of Object.entries<MenuBarEntry[]>($menuBarLayout)) {
    value.forEach((entry) =>
      //@ts-ignore
      entries.push(menuBarEntryToMenuListEntry(key, entry))
    )
  }
}

// TODO: Apparently, Safari does not support the Keyboard.lock() API but does relax its authority over certain keyboard shortcuts in fullscreen mode, which we should take advantage of
const accelKey = PLATFORM === "Mac" ? "Command" : "Control"
const LOCK_REQUIRING_SHORTCUTS = [
  [accelKey, "KeyW"],
  [accelKey, "KeyN"],
  [accelKey, "Shift", "KeyN"],
  [accelKey, "KeyT"],
  [accelKey, "Shift", "KeyT"],
]

const shortcutRequiresLock = (shortcut: LayoutKeysGroup): boolean => {
  const shortcutKeys = shortcut.map((keyWithLabel) => keyWithLabel.key)

  // If this shortcut matches any of the browser-reserved shortcuts
  return LOCK_REQUIRING_SHORTCUTS.some((lockKeyCombo) =>
    arraysEqual(shortcutKeys, lockKeyCombo)
  )
}

type MenuListEntry = MenuBarEntry & {
  w_action: () => void
  w_children: MenuListEntry[][]
  isOpen: boolean
  lastEvent?: KeyboardEvent

  // seem unused:
  value: undefined
  disabled: undefined
  font: undefined
}

function menuBarEntryToMenuListEntry(
  layoutTarget: LayoutTarget,
  entry: MenuBarEntry
): MenuListEntry {
  const ed = assert($editor)
  return {
    // From `MenuEntryCommon`
    ...entry,

    // Shared names with fields that need to be converted from the type used in `MenuBarEntry` to that of `MenuListEntry`
    w_action: () =>
      ed.updateLayout(layoutTarget, BigInt(entry.action.widgetId), undefined),

    w_children: entry.children.map((entries) =>
      entries.map((entry) => menuBarEntryToMenuListEntry(layoutTarget, entry))
    ),

    // New fields in `MenuListEntry`
    // @ts-ignore janky TS compiler
    shortcutRequiresLock: entry.shortcut?.keys
      ? // @ts-ignore
        shortcutRequiresLock(entry.shortcut.keys)
      : undefined,

    value: undefined, // unused
    disabled: undefined, // unused
    font: undefined, // unused
    isOpen: false,
  }
}

function clickEntry(menuListEntry: MenuListEntry, e: MouseEvent) {
  // If there's no menu to open, trigger the action but don't try to open its non-existant children
  if (!menuListEntry.children || menuListEntry.children.length === 0) {
    if (menuListEntry.action && !menuListEntry.disabled)
      menuListEntry.w_action()
  } else {
    // Focus the target so that keyboard inputs are sent to the dropdown
    ;(e.target as HTMLElement | undefined)?.focus()

    menuListEntry.isOpen = true
    entries = entries // trigger update
  }
}

let self: HTMLDivElement | undefined

function unFocusEntry(menuListEntry: MenuListEntry, e: FocusEvent) {
  const blurTarget = (e.target as HTMLElement | undefined)?.closest(
    "[data-menu-bar-input]"
  )

  if (blurTarget !== self) {
    menuListEntry.isOpen = false
    entries = entries // trigger update
  }
}
</script>

<div class="menu-bar-input" data-menu-bar-input bind:this="{self}">
  {#each entries as entry (entry.label)}
    <div class="entry-container">
      <button
        on:click="{(e) => clickEntry(entry, e)}"
        on:blur="{(e) => unFocusEntry(entry, e)}"
        on:keydown="{(e) => (entry.lastEvent = e)}"
        class="entry"
        class:open="{entry.isOpen}"
        data-floating-menu-spawner="{entry.children && entry.children.length > 0
          ? ''
          : 'no-hover-transfer'}"
      >
        {#if entry.icon}
          <IconLabel icon="{entry.icon}" />
        {/if}
        {#if entry.label}
          {entry.label}
        {/if}
      </button>
      {#if entry.children && entry.children.length > 0}
        <MenuList
          open="{entry.isOpen}"
          entries="{entry.children}"
          direction="Bottom"
          minWidth="{240}"
          drawIcon="{true}"
          keydownEvent="{entry.lastEvent}"
        />
      {/if}
    </div>
  {/each}
</div>

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
