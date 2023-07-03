<script lang="ts">
import { listWords } from './lib/server_api';
import Word from './Word.svelte';

export let category_id;
let words_response_p = null;
$: {
    if (category_id) {
        words_response_p = listWords(category_id);
    }
}
</script>

<div class="words">
  {#if words_response_p}
    <div>Category id: {category_id}</div>

    {#await words_response_p}
      <p>Loading</p>
    {:then words_response}
      {#each words_response.words as word}
        <Word word={word} />
      {:else}
        <p>Empty category</p>
      {/each}
    {/await}
  {/if}
</div>

<style>
.words {
  flex-grow: 1;
}
</style>
