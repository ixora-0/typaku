<script lang="ts">
  import { tick } from "svelte";
  import { CharStatus } from "./Word";
  import type WordData from "./Word";
  import type { Position } from "./types";

  export let word_data: WordData;
  export let caret_position: Position | undefined = undefined;

  // have to bind all letters
  // https://github.com/sveltejs/svelte/issues/4570
  let letter_elems: HTMLElement[] = [];

  let rect: DOMRect;

  async function update_caret_position() {
    // wait to render the new typed character, before getting bounding rect
    // which ensure correct caret position when wrong character is typed
    await tick();

    if (word_data.next_char === undefined) {
      // caret at right of last letter
      rect = letter_elems[word_data.next_char_idx - 1]?.getBoundingClientRect();
      if (rect !== undefined) {
        caret_position = { x: rect.right, y: rect.top };
      }
    } else {
      // caret at left of next letter
      rect = letter_elems[word_data.next_char_idx]?.getBoundingClientRect();
      if (rect !== undefined) {
        caret_position = { x: rect.left, y: rect.top };
      }
    }
  }

  // run update_caret_position whenever word_data.next_char changes
  $: {
    word_data.next_char;
    update_caret_position();
  }
</script>

<div class="m-1 decoration-error-500 {word_data.is_wrong ? 'underline' : ''}">
  {#each word_data.display as { char, status }, i}
    <span
      class={(function () {
        switch (status) {
          case CharStatus.Remaining:
            return "text-token";
          case CharStatus.Correct:
            return "text-success-500-400-token";
          case CharStatus.Incorrect:
            return "text-error-500-400-token";
          case CharStatus.Extra:
            return "text-error-400-500-token";
        }
      })() + " m-0"}
      bind:this={letter_elems[i]}>{char}</span
    >
  {/each}
</div>
