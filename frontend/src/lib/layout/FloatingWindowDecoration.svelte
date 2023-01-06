<div class="floating-menu-decor">
  <!-- <div class="tail"></div> -->
  <div class="floating-menu-content">
    <slot />
  </div>
</div>

<style lang="scss">
.floating-menu-decor {
  // Floating menus begin at a z-index of 1000
  isolation: isolate;
  z-index: 10;
  --floating-menu-content-offset: 0;
  --floating-menu-content-border-radius: 4px;

  // .tail {
  // 	width: 0;
  // 	height: 0;
  // 	border-style: solid;
  // 	// Put the tail above the floating menu's shadow
  // 	z-index: 10;
  // 	// Draw over the application without being clipped by the containing panel's `overflow: hidden`
  // 	position: fixed;
  // }
}

.floating-menu-content {
  background: rgba(var(--color-2-mildblack-rgb), 0.95);
  box-shadow: rgba(var(--color-0-black-rgb), 50%) 0 2px 4px;
  border-radius: var(--floating-menu-content-border-radius);
  color: var(--color-e-nearwhite);
}
</style>
