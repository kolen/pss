<script lang="ts">
import { listWords } from './lib/server_api';
import Word from './Word.svelte';

export let category;
let words_response_p = null;
$: {
    if (category) {
        words_response_p = listWords(category.id);
    }
}
</script>

<div class="words">
  {#if words_response_p}
    <div class="pane-header category-toolbar">
      <span class="category-name">Категория: {category.name}</span>
    </div>

    {#await words_response_p}
      <p>Loading</p>
    {:then words_response}
      <table class="ui-table">
        <thead>
          <tr>
            <th></th>
            <th>Слово</th>
            <th>Добавлено</th>
          </tr>
        </thead>
        {#each words_response.words as word}
          <Word word={word} />
        {:else}
          <p>Empty category</p>
        {/each}
      </table>
    {/await}
  {/if}
</div>

<style>
.words {
  flex-grow: 1;
  outline: 1px solid var(--ui-border-color);
}

table {
  width: 100%;
}

table th:nth-child(1) {
  width: 25px;
}

table th:nth-child(2) {
  width: auto;
}

table th:nth-child(3) {
  width: 130px;
}

.category-name {
  font-weight: bold;
  font-size: 80%;
}
</style>
