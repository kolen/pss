<script lang="ts">
import { createEventDispatcher } from 'svelte';
const dispatch = createEventDispatcher();
export let category;
export let selected = false;

function handleClick() {
    console.log('handleClick');
    if (!selected) {
        dispatch('select', {category_id: category.id});
    }
}
</script>

<li on:click={handleClick} class:selected>
  {category.name}
</li>

<style>
.selected {
  font-weight: bold;
}
</style>
