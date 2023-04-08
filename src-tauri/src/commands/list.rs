use super::super::persistence::List;

#[tauri::command]
pub fn list_all() -> Vec<List> {
  return List::all();
}

#[tauri::command]
pub fn list_create(name: String, position: u32, list_group_uuid: Option<String>) -> List {
  return List::create(name, position, list_group_uuid);
}

#[tauri::command]
pub fn list_update(lists: Vec<List>) {
  List::update(lists);
}

#[tauri::command]
pub fn list_delete(uuid: String) {
  List::delete(uuid);
}
