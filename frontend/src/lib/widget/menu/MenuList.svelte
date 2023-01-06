<script lang="ts">
import type { MenuBarEntry } from "graphite-frontend-glue/editor_types"
import { createEventDispatcher } from "svelte"
import IconLabel from "$lib/widget/labels/IconLabel.svelte"
import SeparatorSimple from "../labels/SeparatorSimple.svelte"
import FloatingWindowDecoration from "$lib/layout/FloatingWindowDecoration.svelte"
// import type../widget/labels/IconLabel.sveltert FloatingMenu from "./FloatingMenu.svelte"

export let open: boolean
export let entries: Array<Array<MenuBarEntry>>
export let drawIcon: boolean = false

let forceOpen: boolean = false

$: show = open || forceOpen

let activeEntry: MenuBarEntry | undefined = undefined

// export let minHeight: number = 0
// export let tooltip: string | undefined = undefined

// todo: double click to activate
// todo: show submenu
function onClick(entry: MenuBarEntry) {}

function onEnter(entry: MenuBarEntry) {
  if (activeEntry != undefined) activeEntry = entry
}

function onLeave(entry: MenuBarEntry) {}

function isEntryOpen(entry: MenuBarEntry): boolean {
  return entry == activeEntry
}
</script>

<FloatingWindowDecoration>
  <div
    class="menu-list"
    class:hidden="{!show}"
    on:pointerenter="{() => {
      forceOpen = true
    }}"
    on:pointerleave="{() => {
      forceOpen = false
    }}"
  >
    {#each entries as section, sectionIndex}
      {#if sectionIndex > 0}
        <SeparatorSimple />
      {/if}
      {#each section as entry (entry.label)}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div
          class="row flex flex-row"
          class:open="{isEntryOpen(entry)}"
          class:active="{activeEntry && entry.label === activeEntry.label}"
          style="height:20px;"
          on:click="{() => onClick(entry)}"
          on:pointerenter="{() => onEnter(entry)}"
          on:pointerleave="{() => onLeave(entry)}"
        >
          {#if drawIcon}
            {#if entry.icon}
              <div class="entry-icon">
                <IconLabel icon="{entry.icon}" />
              </div>
            {:else}
              <div class="no-icon"></div>
            {/if}
          {/if}

          <span class="entry-label">{entry.label}</span>

          <!-- <UserInputLabel v-if="entry.shortcut?.keys.length" keysWithLabelsGroups="{[(entry.shortcut).keys]}" requiresLock="{entry.shortcutRequiresLock}" /> -->

          {#if entry.children.length > 0}
            <!-- todo: add icons -->
            <div class="submenu-arrow"></div>
          {:else}
            <div class="no-submenu-arrow"></div>
          {/if}

          <!-- <MenuList
          v-if="entry.children"
          @naturalWidth="(newNaturalWidth: number) => $emit('naturalWidth', newNaturalWidth)"
          :open="entry.ref?.open || false"
          :direction="'TopRight'"
          :entries="entry.children"
          v-bind="{ minWidth, drawIcon, scrollableY }"
          :ref="(ref: MenuListInstance): void => (ref && (entry.ref = ref), undefined)"
        /> -->
        </div>
      {/each}
    {/each}
  </div>
</FloatingWindowDecoration>

<style lang="scss">
.menu-list {
  padding: 4px 0;
}

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

  .entry-icon {
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

    .entry-icon {
      fill: var(--color-f-white);
    }
  }

  &.active {
    background: var(--color-e-nearwhite);
    color: var(--color-2-mildblack);

    .entry-icon {
      fill: var(--color-2-mildblack);
    }
  }

  &.disabled {
    color: var(--color-8-uppergray);

    &:hover {
      background: none;
    }

    fill: var(--color-8-uppergray);
  }
}
</style>
