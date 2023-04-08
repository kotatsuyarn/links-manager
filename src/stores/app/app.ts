import { writable } from 'svelte/store';

export const appStore = writable<{
  listGroups: IListGroup[];
  lists: IList[];
  selectedList: IList;
  tasks: ITask[];
  selectedTask: ITask;
}>({
  listGroups: [],
  lists: [],
  selectedList: null,
  tasks: [],
  selectedTask: null,
});
