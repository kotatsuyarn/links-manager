import { invoke } from '@tauri-apps/api';

export function all(): Promise<ITask[]> {
  return invoke<ITask[]>('task_all')
}

export function taskAllBelongToList(listUuid: string): Promise<ITask[]> {
  return invoke<ITask[]>('task_all_belong_to_list', { listUuid })
}

export function create(
  title: string,
  description: string,
  url: string,
  position: number,
  listUuid: string
): Promise<ITask> {
  return invoke<ITask>('task_create', {
    title,
    description,
    url,
    position,
    listUuid,
  });
}

export function update(tasks: ITask[]): Promise<void> {
  return invoke<void>('task_update', { tasks });
}

export function remove(uuid: uuid): Promise<void> {
  return invoke<void>('task_delete', { uuid });
}