<script lang="ts">
import { listWords } from './lib/server_api';
import Word from './Word.svelte';

export let category_id;
let words_p = null;
$: {
    if (category_id) {
        words_p = listWords(category_id);
    }
}
</script>

<div>
  <div>Category id: {category_id}</div>

  {#if words_p}
    {#await words_p}
    {:then words}
      {#each words as word}
        <Word word={word} />
      {/each}
    {/await}
  {/if}
</div>
