<script lang="ts">
import type {
  LayoutTarget,
  MenuBarEntry,
} from "graphite-frontend-glue/editor_types"
import IconLabel from "$lib/widget/labels/IconLabel.svelte"
import MenuList from "./MenuList.svelte"
import { setContext } from "svelte"

export let entry: MenuBarEntry
export let layoutTarget: LayoutTarget

setContext("layoutTarget", layoutTarget)

export let opened: string | undefined
$: open = opened == entry.label
</script>

<div class="relative h-full flex">
  {#if open}
    <div class="popup absolute min-w-[240px]">
      <MenuList open="{open}" entries="{entry.children}" drawIcon="{true}" />
    </div>
  {/if}
  <button
    on:focus="{() => {
      opened = entry.label
    }}"
    on:blur="{() => {
      opened = undefined
    }}"
    on:pointerenter="{() => {
      if (opened != undefined) opened = entry.label
    }}"
    class="entry"
    class:open="{open}"
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
</div>

<style lang="scss">
.popup {
  // todo: make sure it stays on screen
  top: calc(100%);
}

.entry {
  display: flex;
  align-items: center;
  white-space: nowrap;
  padding: 0 8px;
  background: none;
  border: 0;
  margin: 0;

  :global(svg) {
    fill: var(--color-e-nearwhite);
  }

  &:hover,
  &.open {
    background: var(--color-6-lowergray);

    :global(svg) {
      fill: var(--color-f-white);
    }

    color: var(--color-f-white);
  }
}
</style>
