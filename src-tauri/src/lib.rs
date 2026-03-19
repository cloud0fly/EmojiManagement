// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod db;
mod models;
mod utils;

use commands::category::*;
use commands::clipboard::*;
use commands::meme::*;

use rusqlite::Connection;
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
            import_image_from_folder,
            get_categories,
            get_memes_by_category,
            update_memes_order,
            copy_image,
            move_memes_to_category,
            create_category,
            update_category_order,
            delete_category
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
