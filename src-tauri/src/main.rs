#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod encryption;
mod utils;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![
         encryption::encrypt_pass,
            encryption::verify_pass,
            encryption::encrypt_data,
            encryption::decrypt_data,
            encryption::set_entry,
            encryption::get_entry,
            encryption::receive_encryption_key,
            encryption::set_encryption_key,
            encryption::delete_entry,
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
