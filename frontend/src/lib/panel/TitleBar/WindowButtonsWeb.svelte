<script lang="ts">
import { maximized } from "$lib/stores"
import { onMount } from "svelte"
import { toggleFullscreen } from "$lib/platform"

import SvgIcon from "$lib/widget/labels/SvgIcon.svelte"
import svg_fullscreen_exit from "$icon/12px-solid/fullscreen-exit.svg?raw"
import svg_fullscreen_enter from "$icon/12px-solid/fullscreen-enter.svg?raw"

let keyboardLockSupported = false
let keyboardLocked = false
onMount(() => {
  const keyboard = (navigator as any).keyboard
  if (keyboard && keyboard.lock) {
    keyboardLockSupported = true
  }
})
</script>

<button
  class="px-2 hover:bg-lowergray hover:text-white fill-nearwhite hover:fill-white"
  on:click="{toggleFullscreen}"
  title="{($maximized ? 'Exit' : 'Enter') + ' Fullscreen (F11)'}"
>
  {#if keyboardLockSupported && !keyboardLocked}
    <!-- todo: make this a popup message -->
    <span class="mr-2">Go fullscreen to access all hotkeys</span>
  {/if}

  {#if $maximized}
    <SvgIcon src="{svg_fullscreen_exit}" alt="Exit Fullscreen" />
  {:else}
    <SvgIcon src="{svg_fullscreen_enter}" alt="Enter Fullscreen" />
  {/if}
</button>
