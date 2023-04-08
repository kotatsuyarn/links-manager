use super::super::persistence::Task;

#[tauri::command]
pub fn task_all() -> Vec<Task> {
  return Task::all();
}

#[tauri::command]
pub fn task_all_belong_to_list(list_uuid: String) -> Vec<Task> {
  return Task::all_belong_to_list(list_uuid);
}

#[tauri::command]
pub fn task_create(
  title: String,
  description: String,
  url: String,
  position: u32,
  list_uuid: String
) -> Task {
  return Task::create(title, description, url, position, list_uuid);
}

#[tauri::command]
pub fn task_update(tasks: Vec<Task>) {
  Task::update(tasks);
}

#[tauri::command]
pub fn task_delete(uuid: String) {
  Task::delete(uuid);
}
