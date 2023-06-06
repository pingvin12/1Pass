#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::db::commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::auth, commands::register])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
