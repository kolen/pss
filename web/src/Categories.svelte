<script lang="ts">
import { listCategories } from './lib/server_api';
import Category from './Category.svelte';

let categories_response_p = listCategories();
export let category_id = undefined;
categories_response_p.then((categories) => {
    if (categories.categories.length > 0) {
        category_id = categories.categories[0].id;
    }
});

function handleSelect(event) {
    category_id = event.detail.category_id
}

</script>

<nav>
  <h2>Категории</h2>
  <ul>
    {#await categories_response_p}
      <p>Loading</p>
    {:then categories_response}
      {#each categories_response.categories as category}
        <Category on:select={handleSelect} category={category} selected={category.id == category_id} />
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
