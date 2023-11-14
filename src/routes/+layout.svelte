<script lang="ts">
  import "../app.postcss";
  import { setModeCurrent, getModeOsPrefers } from "@skeletonlabs/skeleton";
  setModeCurrent(getModeOsPrefers());
  import { LightSwitch } from "@skeletonlabs/skeleton";
</script>

<LightSwitch />
<slot />
