<script lang="ts">
  export let title = '';
  export let url = '';
  export let description = '';
</script>

<div>
  <input
    bind:value="{title}"
    class="p-1 border border-gray-300"
    type="text"
    placeholder="title"
  />
  <input
    bind:value="{url}"
    class="p-1 border border-gray-300"
    type="url"
    placeholder="https//:www.example.com"
  />
</div>
<textarea
  bind:value="{description}"
  class="flex-grow p-1 border border-gray-300 resize-none"
  placeholder="description"
/>