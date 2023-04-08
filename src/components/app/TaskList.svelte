<script lang="ts">
	import { appStore } from 'src/stores/app';
	import { open } from '@tauri-apps/api/shell';
  import { TaskInvokes } from 'src/invokes';

	function validUrl(url: string): boolean {
		return url.startsWith('https://') || url.startsWith('http://');
	}

  function openUrl(url: string): void {
    if (!validUrl(url)) {
      return;
    }
    open(url);
  }

  function select(task: ITask): void {
    $appStore.selectedTask = task;
  }

  async function remove(task: ITask): Promise<void> {
    const before = $appStore.tasks.slice(0, task.position);
    const after = $appStore.tasks.slice(task.position + 1);
    after.forEach((l, index) => {
      l.position = task.position + index;
    });
    await TaskInvokes.update(after);
    await TaskInvokes.remove(task.uuid);
    $appStore.tasks = before.concat(after);
  }
</script>

<ul class="p-2 flex-grow flex gap-2 flex-col overflow-y-auto">
  {#each $appStore.tasks as task (task.uuid)}
    <li
      class="p-2 flex justify-between items-center border border-gray-300 cursor-pointer hover:bg-gray-100"
      on:click="{() => select(task)}"
    >
      <div class="flex gap-4 items-center">
        <span>|||</span>
        {#if !validUrl(task.url)}
          <span>{task.title}</span>
        {:else}
          <div
            class="text-blue-800 active:bg-gray-200"
            on:click|stopPropagation="{() => openUrl(task.url)}"
          >
            {task.title}
          </div>
        {/if}
      </div>

      <button
        class="w-5 h-5 flex justify-center items-center hover:bg-gray-300 rounded"
        on:click|stopPropagation="{() => remove(task)}"
      >
        x
      </button>
    </li>
  {:else}
    <li>Empty...</li>
  {/each}
</ul>