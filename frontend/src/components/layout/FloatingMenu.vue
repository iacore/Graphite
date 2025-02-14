<template>
	<div class="floating-menu" :class="[direction.toLowerCase(), type.toLowerCase()]">
		<div class="tail" v-if="displayTail" ref="tail"></div>
		<div class="floating-menu-container" v-if="displayContainer" ref="floatingMenuContainer">
			<LayoutCol class="floating-menu-content" :style="{ minWidth: minWidthStyleValue }" :scrollableY="scrollableY" ref="floatingMenuContent" data-floating-menu-content>
				<slot></slot>
			</LayoutCol>
		</div>
	</div>
</template>

<style lang="scss">
.floating-menu {
	position: absolute;
	width: 0;
	height: 0;
	display: flex;
	// Floating menus begin at a z-index of 1000
	z-index: 1000;
	--floating-menu-content-offset: 0;
	--floating-menu-content-border-radius: 4px;

	&.bottom {
		--floating-menu-content-border-radius: 0 0 4px 4px;
	}

	.tail {
		width: 0;
		height: 0;
		border-style: solid;
		// Put the tail above the floating menu's shadow
		z-index: 10;
		// Draw over the application without being clipped by the containing panel's `overflow: hidden`
		position: fixed;
	}

	.floating-menu-container {
		display: flex;

		.floating-menu-content {
			background: rgba(var(--color-2-mildblack-rgb), 0.95);
			box-shadow: rgba(var(--color-0-black-rgb), 50%) 0 2px 4px;
			border-radius: var(--floating-menu-content-border-radius);
			color: var(--color-e-nearwhite);
			font-size: inherit;
			padding: 8px;
			z-index: 0;
			// Draw over the application without being clipped by the containing panel's `overflow: hidden`
			position: fixed;
		}
	}

	&.dropdown {
		&.top {
			width: 100%;
			left: 0;
			top: 0;
		}

		&.bottom {
			width: 100%;
			left: 0;
			bottom: 0;
		}

		&.left {
			height: 100%;
			top: 0;
			left: 0;
		}

		&.right {
			height: 100%;
			top: 0;
			right: 0;
		}

		&.topleft {
			top: 0;
			left: 0;
			margin-top: -4px;
		}

		&.topright {
			top: 0;
			right: 0;
			margin-top: -4px;
		}

		&.topleft {
			bottom: 0;
			left: 0;
			margin-bottom: -4px;
		}

		&.topright {
			bottom: 0;
			right: 0;
			margin-bottom: -4px;
		}
	}

	&.top.dropdown .floating-menu-container,
	&.bottom.dropdown .floating-menu-container {
		justify-content: left;
	}

	&.popover {
		--floating-menu-content-offset: 10px;
		--floating-menu-content-border-radius: 4px;
	}

	&.cursor .floating-menu-container .floating-menu-content {
		background: none;
		box-shadow: none;
		border-radius: 0;
		padding: 0;
	}

	&.center {
		justify-content: center;
		align-items: center;

		> .floating-menu-container > .floating-menu-content {
			transform: translate(-50%, -50%);
		}
	}

	&.top,
	&.bottom {
		flex-direction: column;
	}

	&.top .tail {
		border-width: 8px 6px 0 6px;
		border-color: rgba(var(--color-2-mildblack-rgb), 0.95) transparent transparent transparent;
		margin-left: -6px;
		margin-bottom: 2px;
	}

	&.bottom .tail {
		border-width: 0 6px 8px 6px;
		border-color: transparent transparent rgba(var(--color-2-mildblack-rgb), 0.95) transparent;
		margin-left: -6px;
		margin-top: 2px;
	}

	&.left .tail {
		border-width: 6px 0 6px 8px;
		border-color: transparent transparent transparent rgba(var(--color-2-mildblack-rgb), 0.95);
		margin-top: -6px;
		margin-right: 2px;
	}

	&.right .tail {
		border-width: 6px 8px 6px 0;
		border-color: transparent rgba(var(--color-2-mildblack-rgb), 0.95) transparent transparent;
		margin-top: -6px;
		margin-left: 2px;
	}

	&.top .floating-menu-container {
		justify-content: center;
		margin-bottom: var(--floating-menu-content-offset);
	}

	&.bottom .floating-menu-container {
		justify-content: center;
		margin-top: var(--floating-menu-content-offset);
	}

	&.left .floating-menu-container {
		align-items: center;
		margin-right: var(--floating-menu-content-offset);
	}

	&.right .floating-menu-container {
		align-items: center;
		margin-left: var(--floating-menu-content-offset);
	}
}
</style>

<script lang="ts">
import { defineComponent, nextTick, type PropType } from "vue";

import LayoutCol from "@/components/layout/LayoutCol.vue";

export type MenuDirection = "Top" | "Bottom" | "Left" | "Right" | "TopLeft" | "TopRight" | "BottomLeft" | "BottomRight" | "Center";
export type MenuType = "Popover" | "Dropdown" | "Dialog" | "Cursor";

const POINTER_STRAY_DISTANCE = 100;

export default defineComponent({
	emits: ["update:open", "naturalWidth"],
	props: {
		open: { type: Boolean as PropType<boolean>, required: true },
		type: { type: String as PropType<MenuType>, required: true },
		direction: { type: String as PropType<MenuDirection>, default: "Bottom" },
		windowEdgeMargin: { type: Number as PropType<number>, default: 6 },
		scrollableY: { type: Boolean as PropType<boolean>, default: false },
		minWidth: { type: Number as PropType<number>, default: 0 },
		escapeCloses: { type: Boolean as PropType<boolean>, default: true },
		strayCloses: { type: Boolean as PropType<boolean>, default: true },
	},
	data() {
		// The resize observer is attached to the floating menu container, which is the zero-height div of the width of the parent element's floating menu spawner.
		// Since CSS doesn't let us make the floating menu (with `position: fixed`) have a 100% width of this container, we need to use JS to observe its size and
		// tell the floating menu content to use it as a min-width so the floating menu is at least the width of the parent element's floating menu spawner.
		// This is the opposite concern of the natural width measurement system, which gets the natural width of the floating menu content in order for the
		// spawner widget to optionally set its min-size to the floating menu's natural width.
		const containerResizeObserver = new ResizeObserver((entries: ResizeObserverEntry[]) => {
			this.resizeObserverCallback(entries);
		});

		return {
			measuringOngoing: false,
			measuringOngoingGuard: false,
			minWidthParentWidth: 0,
			containerResizeObserver,
			pointerStillDown: false,
			workspaceBounds: new DOMRect(),
			floatingMenuBounds: new DOMRect(),
			floatingMenuContentBounds: new DOMRect(),
		};
	},
	computed: {
		minWidthStyleValue() {
			if (this.measuringOngoing) return "0";
			return `${Math.max(this.minWidth, this.minWidthParentWidth)}px`;
		},
		displayTail() {
			return this.open && this.type === "Popover";
		},
		displayContainer() {
			return this.open || this.measuringOngoing;
		},
	},
	// Gets the client bounds of the elements and apply relevant styles to them
	// TODO: Use the Vue :style attribute more whilst not causing recursive updates
	async updated() {
		// Turning measuring on and off both cause the component to change, which causes the `updated()` Vue event to fire extraneous times (hurting performance and sometimes causing an infinite loop)
		if (this.measuringOngoingGuard) return;

		this.positionAndStyleFloatingMenu();
	},
	methods: {
		resizeObserverCallback(entries: ResizeObserverEntry[]) {
			this.minWidthParentWidth = entries[0].contentRect.width;
		},
		positionAndStyleFloatingMenu() {
			if (this.type === "Cursor") return;

			const workspace = document.querySelector("[data-workspace]");
			const floatingMenu: HTMLDivElement | undefined = this.$el;
			const floatingMenuContainer = this.$refs.floatingMenuContainer as HTMLDivElement | undefined;
			const floatingMenuContent: HTMLDivElement | undefined = (this.$refs.floatingMenuContent as typeof LayoutCol | undefined)?.$el;

			if (!workspace || !floatingMenu || !floatingMenuContainer || !floatingMenuContent) return;

			this.workspaceBounds = workspace.getBoundingClientRect();
			this.floatingMenuBounds = floatingMenu.getBoundingClientRect();
			const floatingMenuContainerBounds = floatingMenuContainer.getBoundingClientRect();
			this.floatingMenuContentBounds = floatingMenuContent.getBoundingClientRect();

			const inParentFloatingMenu = Boolean(floatingMenuContainer.closest("[data-floating-menu-content]"));

			if (!inParentFloatingMenu) {
				// Required to correctly position content when scrolled (it has a `position: fixed` to prevent clipping)
				// We use `.style` on a ref (instead of a `:style` Vue binding) because the binding causes the `updated()` hook to call the function we're in recursively forever
				const tailOffset = this.type === "Popover" ? 10 : 0;
				if (this.direction === "Bottom") floatingMenuContent.style.top = `${tailOffset + this.floatingMenuBounds.top}px`;
				if (this.direction === "Top") floatingMenuContent.style.bottom = `${tailOffset + this.floatingMenuBounds.bottom}px`;
				if (this.direction === "Right") floatingMenuContent.style.left = `${tailOffset + this.floatingMenuBounds.left}px`;
				if (this.direction === "Left") floatingMenuContent.style.right = `${tailOffset + this.floatingMenuBounds.right}px`;

				// Required to correctly position tail when scrolled (it has a `position: fixed` to prevent clipping)
				// We use `.style` on a ref (instead of a `:style` Vue binding) because the binding causes the `updated()` hook to call the function we're in recursively forever
				const tail = this.$refs.tail as HTMLElement;
				if (tail && this.direction === "Bottom") tail.style.top = `${this.floatingMenuBounds.top}px`;
				if (tail && this.direction === "Top") tail.style.bottom = `${this.floatingMenuBounds.bottom}px`;
				if (tail && this.direction === "Right") tail.style.left = `${this.floatingMenuBounds.left}px`;
				if (tail && this.direction === "Left") tail.style.right = `${this.floatingMenuBounds.right}px`;
			}

			type Edge = "Top" | "Bottom" | "Left" | "Right";
			let zeroedBorderVertical: Edge | undefined;
			let zeroedBorderHorizontal: Edge | undefined;

			if (this.direction === "Top" || this.direction === "Bottom") {
				zeroedBorderVertical = this.direction === "Top" ? "Bottom" : "Top";

				// We use `.style` on a ref (instead of a `:style` Vue binding) because the binding causes the `updated()` hook to call the function we're in recursively forever
				if (this.floatingMenuContentBounds.left - this.windowEdgeMargin <= this.workspaceBounds.left) {
					floatingMenuContent.style.left = `${this.windowEdgeMargin}px`;
					if (this.workspaceBounds.left + floatingMenuContainerBounds.left === 12) zeroedBorderHorizontal = "Left";
				}
				if (this.floatingMenuContentBounds.right + this.windowEdgeMargin >= this.workspaceBounds.right) {
					floatingMenuContent.style.right = `${this.windowEdgeMargin}px`;
					if (this.workspaceBounds.right - floatingMenuContainerBounds.right === 12) zeroedBorderHorizontal = "Right";
				}
			}
			if (this.direction === "Left" || this.direction === "Right") {
				zeroedBorderHorizontal = this.direction === "Left" ? "Right" : "Left";

				// We use `.style` on a ref (instead of a `:style` Vue binding) because the binding causes the `updated()` hook to call the function we're in recursively forever
				if (this.floatingMenuContentBounds.top - this.windowEdgeMargin <= this.workspaceBounds.top) {
					floatingMenuContent.style.top = `${this.windowEdgeMargin}px`;
					if (this.workspaceBounds.top + floatingMenuContainerBounds.top === 12) zeroedBorderVertical = "Top";
				}
				if (this.floatingMenuContentBounds.bottom + this.windowEdgeMargin >= this.workspaceBounds.bottom) {
					floatingMenuContent.style.bottom = `${this.windowEdgeMargin}px`;
					if (this.workspaceBounds.bottom - floatingMenuContainerBounds.bottom === 12) zeroedBorderVertical = "Bottom";
				}
			}

			// Remove the rounded corner from the content where the tail perfectly meets the corner
			if (this.type === "Popover" && this.windowEdgeMargin === 6 && zeroedBorderVertical && zeroedBorderHorizontal) {
				// We use `.style` on a ref (instead of a `:style` Vue binding) because the binding causes the `updated()` hook to call the function we're in recursively forever
				switch (`${zeroedBorderVertical}${zeroedBorderHorizontal}`) {
					case "TopLeft":
						floatingMenuContent.style.borderTopLeftRadius = "0";
						break;
					case "TopRight":
						floatingMenuContent.style.borderTopRightRadius = "0";
						break;
					case "BottomLeft":
						floatingMenuContent.style.borderBottomLeftRadius = "0";
						break;
					case "BottomRight":
						floatingMenuContent.style.borderBottomRightRadius = "0";
						break;
					default:
						break;
				}
			}
		},
		// To be called by the parent component. Measures the actual width of the floating menu content element and returns it in a promise.
		async measureAndEmitNaturalWidth(): Promise<void> {
			// Wait for the changed content which fired the `updated()` Vue event to be put into the DOM
			await nextTick();

			// Wait until all fonts have been loaded and rendered so measurements of content involving text are accurate
			// API is experimental but supported in all browsers - https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/ready
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			await (document as any).fonts.ready;

			// Make the component show itself with 0 min-width so it can be measured, and wait until the values have been updated to the DOM
			this.measuringOngoing = true;
			this.measuringOngoingGuard = true;
			await nextTick();

			// Measure the width of the floating menu content element, if it's currently visible
			// The result will be `undefined` if the menu is invisible, perhaps because an ancestor component is hidden with a falsy `v-if` condition
			const floatingMenuContent: HTMLDivElement | undefined = (this.$refs.floatingMenuContent as typeof LayoutCol | undefined)?.$el;
			const naturalWidth = floatingMenuContent?.clientWidth;

			// Turn off measuring mode for the component, which triggers another call to the `updated()` Vue event, so we can turn off the protection after that has happened
			this.measuringOngoing = false;
			await nextTick();
			this.measuringOngoingGuard = false;

			// Emit the measured natural width to the parent
			if (naturalWidth !== undefined && naturalWidth >= 0) {
				this.$emit("naturalWidth", naturalWidth);
			}
		},
		pointerMoveHandler(e: PointerEvent) {
			// This element and the element being hovered over
			const self = this.$el as HTMLDivElement | undefined;
			const target = e.target as HTMLElement | undefined;

			// Get the spawner element (that which is clicked to spawn this floating menu)
			// Assumes the spawner is a sibling of this FloatingMenu component
			const ownSpawner: HTMLElement | undefined = self?.parentElement?.querySelector(":scope > [data-floating-menu-spawner]") || undefined;
			// Get the spawner element containing whatever element the user is hovering over now, if there is one
			const targetSpawner: HTMLElement | undefined = target?.closest("[data-floating-menu-spawner]") || undefined;

			// HOVER TRANSFER
			// Transfer from this open floating menu to a sibling floating menu if the pointer hovers to a valid neighboring floating menu spawner
			this.hoverTransfer(self, ownSpawner, targetSpawner);

			// POINTER STRAY
			// Close the floating menu if the pointer has strayed far enough from its bounds (and it's not hovering over its own spawner)
			const notHoveringOverOwnSpawner = ownSpawner !== targetSpawner;
			if (this.strayCloses && notHoveringOverOwnSpawner && this.isPointerEventOutsideFloatingMenu(e, POINTER_STRAY_DISTANCE)) {
				// TODO: Extend this rectangle bounds check to all submenu bounds up the DOM tree since currently submenus disappear
				// TODO: with zero stray distance if the cursor is further than the stray distance from only the top-level menu
				this.$emit("update:open", false);
			}

			// Clean up any messes from lost pointerup events
			const eventIncludesLmb = Boolean(e.buttons & 1);
			if (!this.open && !eventIncludesLmb) {
				this.pointerStillDown = false;
				window.removeEventListener("pointerup", this.pointerUpHandler);
			}
		},
		hoverTransfer(self: HTMLDivElement | undefined, ownSpawner: HTMLElement | undefined, targetSpawner: HTMLElement | undefined): void {
			// Algorithm pseudo-code to detect and transfer to hover-transferrable floating menu spawners
			// Accompanying diagram: <https://files.keavon.com/-/SpringgreenKnownXantus/capture.png>
			//
			// Check our own parent for descendant spawners
			// Filter out ourself and our children
			// Filter out all with a different distance than our own distance from the currently-being-checked parent
			// How many left?
			//     None -> go up a level and repeat
			//     Some -> is one of them the target?
			//         Yes -> click it and terminate
			//         No -> do nothing and terminate

			// Helper function that gets used below
			const getDepthFromAncestor = (item: Element, ancestor: Element): number | undefined => {
				let depth = 1;

				let parent = item.parentElement || undefined;
				while (parent) {
					if (parent === ancestor) return depth;

					parent = parent.parentElement || undefined;
					depth += 1;
				}

				return undefined;
			};

			// A list of all the descendant spawners: the spawner for this floating menu plus any spawners belonging to widgets inside this floating menu
			const ownDescendantMenuSpawners = Array.from(self?.parentElement?.querySelectorAll("[data-floating-menu-spawner]") || []);

			// Start with the parent of the spawner for this floating menu and keep widening the search for any other valid spawners that are hover-transferrable
			let currentAncestor = (targetSpawner && ownSpawner?.parentElement) || undefined;
			while (currentAncestor) {
				const ownSpawnerDepthFromCurrentAncestor = ownSpawner && getDepthFromAncestor(ownSpawner, currentAncestor);
				const currentAncestor2 = currentAncestor; // This duplicate variable avoids an ESLint warning

				// Get the list of descendant spawners and filter out invalid possibilities for spawners that are hover-transferrable
				const listOfDescendantSpawners = Array.from(currentAncestor?.querySelectorAll("[data-floating-menu-spawner]") || []);
				const filteredListOfDescendantSpawners = listOfDescendantSpawners.filter((item: Element): boolean => {
					// Filter away ourself and our descendants
					const notOurself = !ownDescendantMenuSpawners.includes(item);
					// And filter away unequal depths from the current ancestor
					const notUnequalDepths = notOurself && getDepthFromAncestor(item, currentAncestor2) === ownSpawnerDepthFromCurrentAncestor;
					// And filter away elements that explicitly disable hover transfer
					return notUnequalDepths && !(item as HTMLElement).getAttribute?.("data-floating-menu-spawner")?.includes("no-hover-transfer");
				});

				// If none were found, widen the search by a level and keep trying (or stop looping if the root was reached)
				if (filteredListOfDescendantSpawners.length === 0) {
					currentAncestor = currentAncestor?.parentElement || undefined;
				}
				// Stop after the first non-empty set was found
				else {
					const foundTarget = filteredListOfDescendantSpawners.find((item: Element): boolean => item === targetSpawner);
					// If the currently hovered spawner is one of the found valid hover-transferrable spawners, swap to it by clicking on it
					if (foundTarget) {
						this.$emit("update:open", false);
						(foundTarget as HTMLElement).click();
					}

					// In either case, we are done searching
					break;
				}
			}
		},
		keyDownHandler(e: KeyboardEvent) {
			if (this.escapeCloses && e.key.toLowerCase() === "escape") {
				this.$emit("update:open", false);
			}
		},
		pointerDownHandler(e: PointerEvent) {
			// Close the floating menu if the pointer clicked outside the floating menu (but within stray distance)
			if (this.isPointerEventOutsideFloatingMenu(e)) {
				this.$emit("update:open", false);

				// Track if the left pointer button is now down so its later click event can be canceled
				const eventIsForLmb = e.button === 0;
				if (eventIsForLmb) this.pointerStillDown = true;
			}
		},
		pointerUpHandler(e: PointerEvent) {
			const eventIsForLmb = e.button === 0;
			if (this.pointerStillDown && eventIsForLmb) {
				// Clean up self
				this.pointerStillDown = false;
				window.removeEventListener("pointerup", this.pointerUpHandler);
				// Prevent the click event from firing, which would normally occur right after this pointerup event
				window.addEventListener("click", this.clickHandlerCapture, true);
			}
		},
		clickHandlerCapture(e: MouseEvent) {
			// Stop the click event from reopening this floating menu if the click event targets the floating menu's button
			e.stopPropagation();
			// Clean up self
			window.removeEventListener("click", this.clickHandlerCapture, true);
		},
		isPointerEventOutsideFloatingMenu(e: PointerEvent, extraDistanceAllowed = 0): boolean {
			// Consider all child menus as well as the top-level one
			const floatingMenu: HTMLDivElement | undefined = this.$el;
			if (!floatingMenu) return true;
			const allContainedFloatingMenus = [...floatingMenu.querySelectorAll("[data-floating-menu-content]")];

			return !allContainedFloatingMenus.find((element) => !this.isPointerEventOutsideMenuElement(e, element, extraDistanceAllowed));
		},
		isPointerEventOutsideMenuElement(e: PointerEvent, element: Element, extraDistanceAllowed = 0): boolean {
			const floatingMenuBounds = element.getBoundingClientRect();

			if (floatingMenuBounds.left - e.clientX >= extraDistanceAllowed) return true;
			if (e.clientX - floatingMenuBounds.right >= extraDistanceAllowed) return true;
			if (floatingMenuBounds.top - e.clientY >= extraDistanceAllowed) return true;
			if (e.clientY - floatingMenuBounds.bottom >= extraDistanceAllowed) return true;

			return false;
		},
	},
	watch: {
		// Called only when `open` is changed from outside this component (with v-model)
		async open(newState: boolean, oldState: boolean) {
			// Switching from closed to open
			if (newState && !oldState) {
				// TODO: Close any other floating menus that may already be open, which can happen using tab navigation and Enter/Space Bar to open

				// Close floating menu if pointer strays far enough away
				window.addEventListener("pointermove", this.pointerMoveHandler);
				// Close floating menu if esc is pressed
				window.addEventListener("keydown", this.keyDownHandler);
				// Close floating menu if pointer is outside (but within stray distance)
				window.addEventListener("pointerdown", this.pointerDownHandler);
				// Cancel the subsequent click event to prevent the floating menu from reopening if the floating menu's button is the click event target
				window.addEventListener("pointerup", this.pointerUpHandler);

				// Floating menu min-width resize observer

				await nextTick();

				const floatingMenuContainer = this.$refs.floatingMenuContainer as HTMLDivElement | undefined;
				if (!floatingMenuContainer) return;

				// Start a new observation of the now-open floating menu
				this.containerResizeObserver.disconnect();
				this.containerResizeObserver.observe(floatingMenuContainer);
			}

			// Switching from open to closed
			if (!newState && oldState) {
				// Clean up observation of the now-closed floating menu
				this.containerResizeObserver.disconnect();

				window.removeEventListener("pointermove", this.pointerMoveHandler);
				window.removeEventListener("keydown", this.keyDownHandler);
				window.removeEventListener("pointerdown", this.pointerDownHandler);
				// The `pointerup` event is removed in `pointerMoveHandler()` and `pointerDownHandler()`
			}
		},
	},
	components: { LayoutCol },
});
</script>
