<script lang="ts">
  import { ListInvokes, TaskInvokes } from 'src/invokes';
  import { appStore } from 'src/stores/app';
  import List from './List.svelte';
  import ListGroup from './ListGroup.svelte';

	let data: Array<IList | IListGroup> = [];
  $: {
    data = $appStore.lists;
  };

	async function selectList(list: IList): Promise<void> {
    $appStore.selectedList = list;
		$appStore.tasks = await TaskInvokes.taskAllBelongToList($appStore.selectedList.uuid);
	}

	async function createList(): Promise<void> {
		const created = await ListInvokes.create('new list', $appStore.lists.length);
		$appStore.lists = $appStore.lists.concat(created);
	}

  async function deleteList(list: IList): Promise<void> {
    const before = $appStore.lists.slice(0, list.position);
    const after = $appStore.lists.slice(list.position + 1);
    after.forEach((l, index) => {
      l.position = list.position + index;
    });
    await ListInvokes.update(after);
    await ListInvokes.remove(list.uuid);
    $appStore.lists = before.concat(after);

    if (list.uuid === $appStore.selectedList?.uuid) {
      $appStore.selectedList = null;
    }
  }
</script>

<nav class="flex flex-col justify-between border border-gray-300 shadow">
  <ul class="w-42 p-2 flex-grow flex flex-col gap-1 border border-gray-300 shadow overflow-y-auto">
    {#each data as listOrListGroup (listOrListGroup.uuid) }
      {#if !('lists' in listOrListGroup)}
        <List
          name="{listOrListGroup.name}"
          selected="{$appStore.selectedList?.uuid === listOrListGroup.uuid}"
          onClick="{() => selectList(listOrListGroup)}"
          onDelete="{() => deleteList(listOrListGroup)}"
        />
      {:else}
        <li>
          <ListGroup name="{listOrListGroup.name}">
            {#each listOrListGroup.lists as list (list.uuid)}
              <List
                name="{list.name}"
                selected="{$appStore.selectedList?.uuid === listOrListGroup.uuid}"
                onClick="{() => selectList(list)}"
                onDelete="{() => deleteList(list)}"
              />
            {/each}
          </ListGroup>
        </li>
      {/if}
    {:else}
      <li>Empty...</li>
    {/each}
  </ul>

  <button
    class="border border-gray-300 shadow"
    on:click="{createList}"
  >
    +
  </button>
</nav>