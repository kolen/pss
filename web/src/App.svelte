<script lang="ts">
import svelteLogo from './assets/svelte.svg';
import Counter from './lib/Counter.svelte';
import Categories from './Categories.svelte';
import Words from './Words.svelte';
import { writable } from 'svelte/store';

let category_id;
</script>

<main>
  <Categories bind:category_id />

  <Words category_id="{category_id}" />

</main>

<div class="card">
  <Counter />
</div>

<style>
main {
  display: flex;
  width: 100%;
}
</style>
