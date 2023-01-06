<script lang="ts">
import { PLATFORM } from "$lib/platform"
import { portfolio } from "$lib/stores"

import AppMenu from "$lib/widget/menu/AppMenu.svelte"
import WindowButtonsWeb from "./TitleBar/WindowButtonsWeb.svelte"
// import WindowButtonsMac from "@/components/window/title-bar/WindowButtonsMac.vue";
// import WindowButtonsWindows from "@/components/window/title-bar/WindowButtonsWindows.vue";
// import WindowTitle from "@/components/window/title-bar/WindowTitle.vue";

let windowTitle: string
$: {
  if (portfolio) {
    const documentIndex = $portfolio.activeDocumentIndex
    const documentName =
      documentIndex && $portfolio.documents[documentIndex]?.displayName
    windowTitle = documentName ? `${documentName} - Graphite` : "Graphite"
  } else {
    windowTitle = "(wasm uninitialized) - Graphite"
  }
}
</script>

<div class="h-8 items-stretch flex">
  <div class="flex-auto flex justify-start">
    {#if PLATFORM === "Mac"}
      TODO:WindowButtonsMac
      <!-- <WindowButtonsMac /> -->
    {:else}
      <AppMenu />
    {/if}
  </div>
  <div class="flex-auto flex justify-center items-center">
    <div>{windowTitle}</div>
  </div>
  {#if PLATFORM === "Windows" || PLATFORM === "Linux"}
    <div class="flex-auto flex justify-end">
      <!-- <WindowButtonsWindows /> -->
      TODO:WindowButtonsWindows
    </div>
  {:else if PLATFORM === "Web"}
    <div class="flex-auto flex justify-end">
      <WindowButtonsWeb />
    </div>
  {:else}
    <!-- Mac's button is on the left -->
  {/if}
</div>
