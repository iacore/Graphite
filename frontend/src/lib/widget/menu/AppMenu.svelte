<script lang="ts">
import type {
  ActionKeys,
  LayoutKeysGroup,
  LayoutTarget,
  MenuBarEntry,
} from "graphite-frontend-glue/editor_types"
import IconLabel from "../labels/IconLabel.svelte"
import { shortcutRequiresLock } from "$lib/utils"

import { onDestroy, onMount } from "svelte"
import { editor, menuBarLayout } from "$lib/stores"
import { PLATFORM } from "$lib/platform"
import { arraysEqual, assert } from "$lib/utils"
import AppMenuButton from "./AppMenuButton.svelte"
import { readable } from "svelte/store"

let openedLabel: string | undefined = undefined
</script>

<div class="flex">
  {#each $menuBarLayout.MenuBar || [] as entry, i (entry.label)}
    <AppMenuButton
      layoutTarget="MenuBar"
      entry="{entry}"
      bind:opened="{openedLabel}"
    />
  {/each}
</div>
