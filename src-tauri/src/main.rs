#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Manager};
mod persistence;
mod commands;
use commands::list::{
  list_all,
  list_create,
  list_update,
  list_delete,
};
use commands::task::{
  task_all,
  task_all_belong_to_list,
  task_create,
  task_update,
  task_delete,
};

fn main() {
  persistence::init_table_if_not_exists();

  tauri::Builder::default()
    .setup(|app| {
      match Manager::get_window(app, "main") {
        Some(window) => {
          window.center()?;
        },
        None => {},
      };
      return Ok(());
    })
    .invoke_handler(
      tauri::generate_handler![
        list_all,
        list_create,
        list_update,
        list_delete,

        task_all,
        task_all_belong_to_list,
        task_create,
        task_update,
        task_delete,
      ]
    )
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
