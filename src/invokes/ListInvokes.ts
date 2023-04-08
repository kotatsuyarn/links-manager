import { invoke } from '@tauri-apps/api';

export function all(): Promise<IList[]> {
  return invoke<IList[]>('list_all')
}

export function create(name: string, position: number): Promise<IList> {
  return invoke<IList>('list_create', {
    name,
    position,
    listGroupUuid: null,
  });
}

export function update(lists: IList[]): Promise<void> {
  return invoke<void>('list_update', { lists });
}

export function remove(uuid: uuid): Promise<void> {
  return invoke<void>('list_delete', { uuid });
}
