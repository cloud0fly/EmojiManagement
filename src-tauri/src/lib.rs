// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod db;
mod commands;

use rusqlite::{Connection};
use std::sync::Mutex;
use tauri::Manager;
struct DbState(Mutex<Connection>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let conn = db::init(app.app_handle())?;
            app.manage(DbState(Mutex::new(conn)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::meme::copy_image,
            commands::meme::import_image_from_folder,
            commands::meme::get_memes_by_category,
            commands::meme::update_memes_order,
            commands::category::get_categories,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
