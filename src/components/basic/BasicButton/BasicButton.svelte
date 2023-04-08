<script lang="ts">
export let stopPropagation = false;
export let disabled = false;
export let onclick: (e: Event) => void = () => {};

function handleClick(e: Event): void {
  if(stopPropagation) e.stopPropagation();
  onclick(e);
}
</script>


<button
  disabled="{disabled}"
  class="px-2 py-1 border border-gray-300 hover:bg-gray-100 shadow rounded"
  on:click="{handleClick}"
>
  <slot></slot>
</button>
