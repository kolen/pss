<script lang="ts">
import { createEventDispatcher } from 'svelte';
const dispatch = createEventDispatcher();
export let category;
export let selected = false;

function handleClick() {
    if (!selected) {
        dispatch('select', {category: category});
    }
}
</script>

<li on:click={handleClick} class:selected>
  {category.name}
</li>

<style>
li {
  padding: 4px 10px;
}

.selected {
  background: var(--ui-selection-color);
}
</style>
