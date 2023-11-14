<script lang="ts">
  import LiveWpm from "$lib/typing/LiveWPM.svelte";
  import TypingArea from "$lib/typing/TypingArea.svelte";
  import TypingAreaLoading from "$lib/typing/TypingAreaLoading.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  async function init_log() {
    await invoke("init_log");
  }
  async function load_data() {
    await invoke("load_data")
      .then((app_data) => console.log(app_data))
      .catch((error) => console.log(error)); // TODO: handle error
  }
  async function get_text(): Promise<string> {
    try {
      return await invoke("gen_txt_from_mc200", { num_words: 50 });
    } catch (error) {
      console.log(error);
      return "Error"; // TODO: handle error
    }
  }

  let typing_area_reset_key = {};
  let reset_button: HTMLElement;
  function reset_typing_area() {
    typing_area_reset_key = {}; // every {} is unique, {} === {} evaluates to false
    reset_button.blur(); // unfocus on the button, allows for quick resetting
  }
</script>

<button
  class="variant-ghost-primary bg-primary-hover-token py-2 px-5 text-2xl rounded-token"
  on:click={init_log}>Init log</button
>
<button
  class="variant-ghost-primary bg-primary-hover-token py-2 px-5 text-2xl rounded-token"
  on:click={load_data}>Load data</button
>
<div class="flex justify-center">
  <LiveWpm />
</div>

{#key typing_area_reset_key}
  {#await get_text()}
    <TypingAreaLoading />
  {:then text}
    <TypingArea init_words={text.split(" ")} />
  {/await}
{/key}
<button
  class="variant-ghost-primary bg-primary-hover-token py-2 px-5 text-xl"
  on:click={reset_typing_area}
  bind:this={reset_button}>Reset</button
>
