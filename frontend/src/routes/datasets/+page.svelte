<script lang="ts">
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<ul>
  {#each data.datasets as dataset}
    <li>
      <strong>{dataset.name}</strong><br />
      {dataset.description}
    </li>
  {/each}
</ul>
