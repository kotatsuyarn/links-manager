<script lang="ts">
	import { ModalContainer } from './stores/modal/components/ModalContainer';
	import { modal } from './stores/modal';
	import { appStore } from 'src/stores/app';
	import { ListInvokes, TaskInvokes } from './invokes';
	import { ListNav, TaskDetail, TaskList, ShowTask } from 'src/components/app';
	
	let renameList = false;
	let creatingTask = false;
	let taskTitle = '';
	let taskUrl = '';
	let taskDescription = '';

	async function main(): Promise<void> {
		$appStore.lists = await ListInvokes.all();
	}

	main();

	async function createTask(): Promise<void> {
		const newTask = await TaskInvokes.create(
			taskTitle,
			taskDescription,
			taskUrl,
			$appStore.tasks.length,
			$appStore.selectedList.uuid
		);
		$appStore.tasks = $appStore.tasks.concat(newTask);
		creatingTask = false;
	}

	async function updateListname(): Promise<void> {
		await ListInvokes.update([$appStore.selectedList]);
		renameList = false;
	}
</script>

<nav class="px-2 py-1 flex justify-between items-center w-full border border-gray-300 shadow">
	<p>Logo</p>
	<div
		class="min-w-22 text-center p-1 bg-gray-300 hover:bg-gray-200 rounded cursor-pointer"
		on:click="{modal.show.settings}"
	>
		Settings
	</div>
</nav>

<main class="flex flex-grow overflow-auto">
	<ListNav />
	
	{#if !!$appStore.selectedList}
		<article class="flex-grow flex flex-col">
			<section class="p-2 flex justify-between items-center">
				<div>
					<div>
						{#if !renameList}
							<span>{$appStore.selectedList.name}</span>
							<span on:click="{() => renameList = true}">(pen for renaming)</span>
						{:else}
							<input
								type="text"
								bind:value="{$appStore.selectedList.name}"
								class="border border-gray-300"
							/>
							<button on:click="{updateListname}">save</button>
						{/if}
					</div>
					<p class="text-sm">
						{$appStore.tasks.length} Task{$appStore.tasks.length !== 1 ? 's' : ''}
					</p>
				</div>

				<div>
					<button
						class="px-2 py-1 border border-gray-300 shadow"
						on:click="{() => creatingTask = true}"
					>
						Create
					</button>
				</div>
			</section>
		
			<section class="flex-grow flex flex-col border border-gray-300 shadow overflow-y-auto">
				{#if creatingTask}
					<div class="p-2 flex-grow flex flex-col gap-2">
						<TaskDetail bind:title="{taskTitle}" bind:url="{taskUrl}" bind:description="{taskDescription}" />
						<div class="flex gap-2 justify-end items-center">
							<button
								class="px-2 py-1 border border-gray-300 shadow"
								on:click="{createTask}"
							>
								save
							</button>
							<button
								class="px-2 py-1 border border-gray-300 shadow"
								on:click="{() => creatingTask = false}"
							>
								back
							</button>
						</div>
					</div>
				{:else if !!$appStore.selectedTask}
					<ShowTask />
				{:else}
					<TaskList />
				{/if}
			</section>
		</article>
	{:else}
		<article class="flex-grow flex flex-col items-center justify-center">
			Emptiness
		</article>
	{/if}
</main>

<ModalContainer />
