<script lang="ts">
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<svelte:head>
  <title>List of datasets</title>
</svelte:head>

{#if data.datasets.length}
  {#each data.datasets as dataset}
    <nav>
      <ul>
        <li>
          <a href="datasets/{dataset.id}"><strong>{dataset.name}</strong></a><br
          />
          {dataset.description}
        </li>
      </ul>
    </nav>
  {/each}
{:else}
  <h1>No datasets yet!</h1>
  <p>Try adding a dataset through the backend.</p>
{/if}
