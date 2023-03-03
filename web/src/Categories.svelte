<script lang="ts">
import { listCategories } from './lib/server_api';
import Category from './Category.svelte';

let categories_p = listCategories();
export let category_id = undefined;
categories_p.then((categories) => {
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
    {#await categories_p}
    {:then categories}
      {#each categories.categories as category}
        <Category on:select={handleSelect} category={category} selected={category.id == category_id} />
      {/each}
    {/await}
  </ul>
</nav>

<style>
ul {
  list-style-type: none;
  padding: 0;
}
</style>
