<script lang="ts">
import { listCategories } from './lib/server_api';
import Category from './Category.svelte';

let categories_response_p = listCategories();
export let category = undefined;
categories_response_p.then((categories) => {
    if (categories.categories.length > 0) {
        category = categories.categories[0];
    }
});

function handleSelect(event) {
    category = event.detail.category
}

</script>

<nav>
  <hgroup class="pane-header">
    <h2 class="ui-header">Категории</h2>
  </hgroup>
  <ul>
    {#await categories_response_p}
      <p>Loading</p>
    {:then categories_response}
      {#each categories_response.categories as i_category}
        <Category on:select={handleSelect} category={i_category} selected={i_category == category} />
      {:else}
        <p>No categories</p>
      {/each}
    {/await}
  </ul>
</nav>

<style>
nav {
  flex-basis: 250px;
  max-width: 250px;
  outline: 1px solid var(--ui-border-color);
}

ul {
  list-style-type: none;
  padding: 0;
  margin: 0;
}
</style>
