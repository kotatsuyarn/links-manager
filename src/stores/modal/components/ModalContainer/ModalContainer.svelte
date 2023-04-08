<script lang="ts">
	import { modal } from '../..';
</script>

{#if $modal}
<aside class="absolute h-screen w-full bg-[rgba(0,0,0,0.5)]">
  <div
    class="absolute right-2 hover:text-gray-300 cursor-pointer"
    on:click|stopPropagation="{modal.close}"
  >
    x
  </div>
  
  <div class="h-full flex flex-col items-center justify-center">
    <svelte:component this="{$modal}" />
  </div>
</aside>
{/if}