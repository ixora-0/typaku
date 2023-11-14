<script lang="ts">
  import { onDestroy } from "svelte";
  import { in_progress, typed_chars, text_progress } from "./stores";
  export let decimal_places = 0;

  let raw_wpm = -1;
  let wpm = -1;

  let init_time: number;
  let frame: number;
  function update() {
    // continue updating while still in progress of typing
    if ($in_progress) {
      frame = requestAnimationFrame(update);
    }
    const current_time = window.performance.now();
    const ms_elapsed = current_time - init_time;
    const mins_elapsed = ms_elapsed / 60_000;
    raw_wpm = $typed_chars / mins_elapsed / 5.0;
    wpm = $text_progress / mins_elapsed / 5.0;
  }
  $: if ($in_progress) {
    init_time = window.performance.now();
    update();
  }
  onDestroy(() => {
    cancelAnimationFrame(frame);
  });
</script>

<div class="text-6xl">
  <span class="px-5">Raw WPM: {raw_wpm >= 0 ? raw_wpm.toFixed(decimal_places) : "N/A"}</span>

  <span class="px-5">WPM: {wpm >= 0 ? wpm.toFixed(decimal_places) : "N/A"}</span>
</div>
