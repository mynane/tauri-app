#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri::api::notification::Notification;

#[tauri::command]
fn close_splashscreen(window: tauri::Window) {
  Notification::new("studio.tauri.example")
  .title("New message")
  .body("You've got a new message.")
  .show().unwrap();
    if let Some(splashscreen) = window.get_window("splashscreen") {
      splashscreen.close().unwrap();
    }

    println!("123123");

    window.get_window("main").unwrap().show().unwrap();
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![close_splashscreen])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
