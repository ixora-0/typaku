<script lang="ts">
  import type { Position } from "./types";

  export let position: Position;
  export let width: String = "2px";
  export let height: String;

  export let duration_seconds = 2;
  export let delay_ms = 250;

  let blink = true;
  let blink_timeout_id: NodeJS.Timeout | null = null;
  async function reset_animation() {
    if (blink_timeout_id !== null) {
      clearTimeout(blink_timeout_id);
    }
    blink = false;
    blink_timeout_id = setTimeout(() => {
      blink = true;
      blink_timeout_id = null;
    }, delay_ms);
  }

  $: if (position) {
    // reset animation when position change
    reset_animation();
  }
</script>

<div
  class="absolute bg-primary-900-50-token {blink ? 'caret-blink' : ''}"
  style:width
  style:left={position.x + "px"}
  style:top={position.y + "px"}
  style:height
  style="--duration:{duration_seconds}s;"
/>

<style>
  .caret-blink {
    animation-name: blink;
    animation-duration: var(--duration);
    animation-iteration-count: infinite;
  }
  @keyframes blink {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0;
    }
  }
</style>
