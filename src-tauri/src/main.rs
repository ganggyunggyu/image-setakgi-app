#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod commands;
mod config;
mod image_ops;
mod output;
mod preview;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::generate_preview,
            commands::convert_all,
            commands::save_preset,
            commands::load_preset
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
