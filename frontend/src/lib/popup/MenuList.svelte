<script lang="ts" context="module">
import IconLabel from "$lib/widget/labels/IconLabel.svelte"
import type { ActionKeys } from "graphite-frontend-glue/editor_types"

export type MenuListEntry = {
  label: string
  icon: undefined | string
  shortcut: undefined | ActionKeys

  action: () => void
  children: MenuListEntry[][]
  isOpen: boolean
  lastEvent?: KeyboardEvent
  font: string | undefined

  // seem unused:
  value: undefined
  disabled: boolean | undefined
}
</script>

<script lang="ts">
import { createEventDispatcher } from "svelte"
import type { MenuDirection } from "./FloatingMenu.svelte"
// import FloatingMenu from "./FloatingMenu.svelte"

export let open: boolean
export let entries: Array<Array<MenuListEntry>>
export let activeEntry: MenuListEntry | undefined = undefined
export let direction: MenuDirection = "Bottom"
export let minWidth: number = 0
export let drawIcon: boolean = false
export let interactive: boolean = false
export let scrollableY: boolean = false
export let minHeight: number = 0
export let tooltip: string | undefined = undefined

export let keydownEvent: undefined | KeyboardEvent

let scroller: HTMLElement
// let floatingMenu: FloatingMenu

function onClick(entry: MenuListEntry) {}

function onEnter(entry: MenuListEntry) {}

function onLeave(entry: MenuListEntry) {}
</script>

<!-- <FloatingMenu
  bind:open="{open}"
  on:naturalWidth
  type="Dropdown"
  windowEdgeMargin="{0}"
  escapeCloses="{false}"
  direction="{direction}"
  minWidth="{minWidth}"
  scrollableY="{scrollableY && zeroHeight}"
  bind:this="{floatingMenu}"
> -->
<div class="floating-menu">
  <!-- If we put the scrollableY on the layoutcol for non-font dropdowns then for some reason it always creates a tiny scrollbar.
		However when we are using the virtual scrolling then we need the layoutcol to be scrolling so we can bind the events without using $refs. -->
  <div bind:this="{scroller}" class:scrollable-y="{scrollableY}">
    {#each entries as section, sectionIndex}
      {#if sectionIndex > 0}
        <!-- <Separator type="List" direction="Vertical" /> -->
        TODO:Separator
      {/if}
      {#each section as entry (entry.label)}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div
          class="row"
          class:open="{entry.isOpen}"
          class:active="{activeEntry && entry.label === activeEntry.label}"
          class:disabled="{entry.disabled}"
          style="height:20px;"
          title="{tooltip}"
          on:click="{() => !entry.disabled && onClick(entry)}"
          on:pointerenter="{() => !entry.disabled && onEnter(entry)}"
          on:pointerleave="{() => !entry.disabled && onLeave(entry)}"
        >
          {#if entry.icon && drawIcon}
            <div class="entry-icon">
              <IconLabel icon="{entry.icon}" />
            </div>
          {/if}
          {#if entry.font}
            <link rel="stylesheet" href="{entry.font}" />
            <!-- <div v-else-if="drawIcon" class="no-icon"></div> -->
          {/if}

          <span
            class="entry-label"
            style:fontFamily="{`${!entry.font ? "inherit" : entry.value}`}"
            >{entry.label}</span
          >

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
</div>

<!-- </FloatingMenu> -->
<style lang="scss">
	.floating-menu {
		position: relative;
	}
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
</style>
