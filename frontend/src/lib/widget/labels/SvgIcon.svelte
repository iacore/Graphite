<svelte:options immutable={true}/>

<script lang="ts">
import { assert } from "$lib/utils"

/**
 * The svg string, NOT the url to the svg
 * Import with `import xxxxx from '$icon/xxxxx/*.svg'
 */
export let src: string
export let alt: string | undefined

const match = /viewBox="(\d+) (\d+) (\d+) (\d+)"/.exec(src)
const [_whole, _x, _y, w, h] = assert(match, 'SVG has no viewBox')

const style = `--svg-width:${w}px;--svg-height:${h}px;`
</script>

<div class="wrapper" title="{alt}" style={style} >{@html src}</div>

<style>
.wrapper {
  line-height: 0;
}
.wrapper :global(svg) {
  display: inline;
  width: var(--svg-width);
  height: var(--svg-height);
}
</style>
