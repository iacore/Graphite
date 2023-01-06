<svelte:options immutable="{true}" />

<script lang="ts">
import SvgIcon from "./SvgIcon.svelte"
import ICONS from "$lib/icons"

/** Icon name. Used to be the Vue Component name */
export let icon: string
export let disabled: boolean = false
export let tooltip: string | undefined

let svgString: string | undefined
$: svgString = ICONS[icon]
</script>

<div class="icon-label" class:disabled="{disabled}">
  {#if svgString}
    <SvgIcon src="{svgString}" alt="{tooltip}" />
  {:else}
    NEXIST '{icon}'
  {/if}
</div>

<style lang="scss">
@import '$lib/_colors.scss';
.icon-label {
  flex: 0 0 auto;
  fill: var(--color-e-nearwhite);

  &.disabled {
    fill: var(--color-8-uppergray);
  }
}
</style>
