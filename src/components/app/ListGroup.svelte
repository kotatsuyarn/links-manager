<script lang="ts">
  export let name: string;

  let show = false;
</script>

<div>
  <div
    class="p-2 inline-block bg-gray-300 cursor-pointer"
    on:click="{() => show = !show}"
  >
    {#if show} v {:else} -&gt; {/if}
    {name}
  </div>

  {#if show}
    <div class="border-l-4 border-gray-300">
      <ul class="px-2 py-1 bg-white">
        <slot></slot>
      </ul>
    </div>
  {/if}
</div>