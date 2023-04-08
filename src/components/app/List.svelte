<script lang="ts">
  export let name: string;
  export let selected: boolean = false;
  export let onClick = () => {};
  export let onDelete = () => {};
</script>

<li
  class="p-2 flex justify-between items-center hover:bg-gray-200 cursor-pointer"
  class:bg-gray-200="{selected}"
  on:click="{onClick}"
>
  <span>{name}</span>

  <button
    class="w-5 h-5 flex justify-center items-center hover:bg-gray-300 rounded"
    on:click|stopPropagation="{onDelete}"
  >
    x
  </button>
</li>