<script lang="ts">
// import Workspace from "$lib/panel/Workspace.svelte"
import TitleBar from "$lib/panel/TitleBar.svelte"
// import StatusBar from "$lib/panel/StatusBar.svelte"
</script>

<div class="root flex flex-col">
  <TitleBar/>

  <div class="flex-grow">
  <!-- <Workspace /> -->
  TODO:Workspace

  </div>
  <!-- <StatusBar /> -->
  TODO:Statusbar  
</div>


<style>
  :root {
    /* text styles */
    @apply bg-mildblack text-nearwhite leading-none;
    @apply overflow-hidden overscroll-none select-none touch-none;
  }
  .root {
    height: 100vh;
  }
</style>
