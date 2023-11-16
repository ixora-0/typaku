<script lang="ts">
  import "../app.postcss";
  import { setModeCurrent, getModeOsPrefers } from "@skeletonlabs/skeleton";
  setModeCurrent(getModeOsPrefers());
  import { LightSwitch } from "@skeletonlabs/skeleton";
  import { initializeStores } from "@skeletonlabs/skeleton";

  // should be in root layout
  // required only once when implementing Skeleton's Drawer, Modal, and Toast stores
  // prevent known issues with SvelteKit SSR
  // https://www.skeleton.dev/utilities/drawers#prerequisites
  initializeStores();
</script>

<LightSwitch />
<slot />
