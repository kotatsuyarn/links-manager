<div class="p-2 bg-white border border-gray-300 shadow rounded">
  <form
    on:submit|preventDefault="{() => console.log('create list')}"
    class="flex flex-col gap-4"
  >
    <p class="flex flex-col">
      <label for="name">name:</label>
      <input
        type="text"
        id="name"
        class="border border-gray-300"
      />
    </p>

    <p class="flex flex-col">
      <label for="description">description:</label>
      <textarea
        id="description"
        class="border border-gray-300 resize-none"
      ></textarea>
    </p>

    <button class="border border-gray-300 shadow">Create</button>
  </form>
</div>