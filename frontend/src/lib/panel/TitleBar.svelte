<script lang="ts">
import { PLATFORM } from "$lib/platform"
import { portfolio } from "$lib/stores"

// import MenuBarInput from "@/components/widgets/inputs/MenuBarInput.vue";
// import WindowButtonsMac from "@/components/window/title-bar/WindowButtonsMac.vue";
// import WindowButtonsWeb from "@/components/window/title-bar/WindowButtonsWeb.vue";
// import WindowButtonsWindows from "@/components/window/title-bar/WindowButtonsWindows.vue";
// import WindowTitle from "@/components/window/title-bar/WindowTitle.vue";

let windowTitle: string
$: {
  if (portfolio) {
    const documentIndex = $portfolio.activeDocumentIndex
    const documentName = documentIndex && $portfolio.documents[documentIndex]?.displayName
    windowTitle = documentName ? `${documentName} - Graphite` : "Graphite"
  } else {
    windowTitle = "(wasm uninitialized) - Graphite"
  }
}
</script>

<div class="height-28px flex flex-row">
  <div class="text-start flex-auto">
    TODO
    <!-- {#if PLATFORM === 'Mac'}
				<WindowButtonsMac />
				{:else}
				<MenuBarInput />
				{/if} -->
  </div>
  <div class="text-center flex-auto">
    {windowTitle}
  </div>
  <div class="text-end flex-auto">
    TODO
    <!-- {#if platform === 'Windows' || platform === 'Linux'}
				<WindowButtonsWindows />
				{:else if platform === 'Web' }
				<WindowButtonsWeb/>
				{:else}
				window buttons here?
			{/if} -->
  </div>
</div>
