#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use onepass::{
    appstate::AppState,
    db::{
        commands::{self},
        database::Database,
    },
};
use tauri::{Manager, State};

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db_manager: Default::default(),
        })
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::auth,
            commands::register,
            commands::me,
            commands::init_hosting,
            commands::get_secrets,
            commands::create_secret,
            commands::delete_secret
        ])
        .setup(|app| {
            let handle: tauri::AppHandle = app.handle();
            let app_state: State<AppState> = handle.state();
            let db_new_pool = Database::new_pool().unwrap();

            *app_state.db_manager.lock().unwrap() = Some(db_new_pool);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
