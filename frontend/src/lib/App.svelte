<script lang="ts">
// import Workspace from "$lib/panel/Workspace.svelte"
import TitleBar from "$lib/panel/TitleBar.svelte"
// import StatusBar from "$lib/panel/StatusBar.svelte"

import { rootElement } from "$lib/stores"
</script>

<div class="root flex flex-col" bind:this="{$rootElement}">
  <TitleBar />

  <div class="flex-grow">
    <!-- <Workspace /> -->
    <div class="bg-checkered text-black h-full">TODO:Workspace</div>
  </div>
  <!-- <StatusBar /> -->
  TODO:Statusbar
</div>

<style>
.root {
  height: 100vh;
}
</style>
