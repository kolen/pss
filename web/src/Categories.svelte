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
  <h2>Категории</h2>
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
  flex-basis: 300px;
}

ul {
  list-style-type: none;
  padding: 0;
}
</style>
