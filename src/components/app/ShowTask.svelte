<script lang="ts">
  import { appStore } from 'src/stores/app';
	import { TaskDetail } from 'src/components/app';
  import { TaskInvokes } from 'src/invokes';

  const task: ITask = structuredClone($appStore.selectedTask);

  async function save(): Promise<void> {
    await TaskInvokes.update([task]);
    $appStore.tasks[task.position] = task;
    $appStore.selectedTask = null;
  }
</script>

<div class="p-2 flex-grow flex gap-2">
  <div class="flex justify-center items-center">
    <button
      class="p-2 border border-gray-300 disabled:opacity-50"
      disabled="{$appStore.selectedTask?.position <= 0}"
      on:click="{() => $appStore.selectedTask = $appStore.tasks[$appStore.selectedTask?.position - 1]}"
    >
      &lt;
    </button>
  </div>
  <div class="flex-grow flex flex-col gap-2">
    <TaskDetail
      bind:title="{task.title}"
      bind:url="{task.url}"
      bind:description="{task.description}"
    />
    <div class="flex gap-2 justify-between items-center">
      <span>{$appStore.selectedTask.position+1}/{$appStore.tasks.length}</span>
      <div>
        <button
          class="px-2 py-1 border border-gray-300 shadow"
          on:click="{save}"
        >
          save
        </button>
        <button
          class="px-2 py-1 border border-gray-300 shadow"
          on:click="{() => $appStore.selectedTask = null}"
        >
          back
        </button>
      </div>
    </div>
  </div>
  <div class="flex justify-center items-center">
    <button
      class="p-2 border border-gray-300 disabled:opacity-50"
      disabled="{$appStore.tasks.length <= $appStore.selectedTask?.position + 1}"
      on:click="{() => $appStore.selectedTask = $appStore.tasks[$appStore.selectedTask?.position + 1]}"
    >
      &gt;
    </button>
  </div>
</div>