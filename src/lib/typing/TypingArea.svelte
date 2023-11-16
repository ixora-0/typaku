<script lang="ts">
  import Word from "./Word.svelte";
  import WordData from "./Word";
  import type { Position } from "./types";
  import Caret from "./Caret.svelte";
  import { tweened, type Tweened } from "svelte/motion";
  import { in_progress, typed_chars, text_progress } from "./stores";

  export let init_words: string[];
  export let caret_motion_duration_ms = 100;
  export let quick_finish = true; // if false, only end after pressing space after the typing last word

  // reset stores when typing area is re-initialized
  in_progress.set(false);
  typed_chars.set(0);
  text_progress.set(0);

  let typed_words: WordData[] = [];
  let current_word: WordData | undefined = new WordData(init_words[0]); // is undefined when there're no more remaining word i.e finished typing
  let current_word_idx = 0;
  let remaining_words: WordData[] = init_words.slice(1).map((word) => new WordData(word));

  // updating text_progress. kinda inefficient
  let all_words: WordData[];
  // also includes remaining_words because we could've made progress in some words, then backspace backwards
  // the progress made in the backspaced word is still considered
  $: all_words =
    current_word === undefined
      ? [...typed_words, ...remaining_words]
      : [...typed_words, current_word, ...remaining_words];
  // spaces are not considered in WordData, have to keep track here
  let typed_spaces = new Array(init_words.length - 1).fill(false);
  $: num_typed_spaces = typed_spaces.filter(Boolean).length;
  $: text_progress.set(
    all_words.reduce((sum, word) => sum + word.progress(), 0) + num_typed_spaces
  );

  // no more remaining word -> finished typing -> not in progress
  $: if (current_word === undefined) {
    in_progress.set(false);
  }

  let caret_position: Position | undefined = undefined;
  let caret_position_tweened: Tweened<Position | undefined>;
  $: if (caret_position === undefined) {
    caret_position_tweened = tweened<Position | undefined>(undefined, {
      duration: caret_motion_duration_ms
    });
  } else {
    caret_position_tweened.set(caret_position);
  }

  function handleKeydown(event: KeyboardEvent) {
    if (current_word === undefined) {
      return;
    }
    if (event.key.length === 1) {
      in_progress.set(true); // there's still a word and we typed something
      typed_chars.update((n) => n + 1); // increment typed_char

      if (event.key === " ") {
        current_word.is_wrong = !current_word.is_correctly_finished();
        typed_words = [...typed_words, current_word];
        // can't use remaining_words.shift() because svelte only updates when assigning
        current_word = remaining_words[0];
        typed_spaces[current_word_idx] = true;
        current_word_idx++;
        remaining_words = remaining_words.slice(1);
      } else {
        current_word.insert_char(event.key);
        current_word = current_word; // svelte only updates when assigning
        if (
          quick_finish &&
          current_word_idx === init_words.length - 1 &&
          current_word.is_correctly_finished()
        ) {
          // test is done
          typed_words = [...typed_words, current_word];
          current_word = undefined;
          current_word_idx++;
          remaining_words = []; // should already be an empty array
        }
      }
    } else if (event.key === "Backspace") {
      // TODO: handle Ctrl+Backspace
      if (!current_word.backspace() && typed_words.length > 0) {
        // move back a word if can't backspace from current and there's a previous word
        remaining_words = [current_word, ...remaining_words];
        current_word = typed_words[typed_words.length - 1];
        current_word.is_wrong = false;
        current_word_idx--;
        typed_words = typed_words.slice(0, typed_words.length - 1);
      } else {
        current_word = current_word; // svelte only updates when assigning
      }
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if $caret_position_tweened && current_word !== undefined}
  <Caret position={$caret_position_tweened} width="2px" height="2rem" />
{/if}

<div class="flex flex-wrap select-none text-2xl">
  {#each typed_words as typed_word}
    <Word word_data={typed_word} />
  {/each}
  {#if current_word !== undefined}
    <Word word_data={current_word} bind:caret_position />
  {/if}
  {#each remaining_words as remaining_word}
    <Word word_data={remaining_word} />
  {/each}
</div>
