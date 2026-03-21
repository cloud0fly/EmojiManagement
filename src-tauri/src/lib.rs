// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod db;
mod models;
mod utils;

use tauri_plugin_log::Target;
use commands::category::*;
use commands::clipboard::*;
use commands::meme::*;

use rusqlite::Connection;
use tauri_plugin_log::TargetKind;
use std::sync::Mutex;

use tauri::menu::{Menu, MenuItem};
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};

struct DbState(Mutex<Connection>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // plugin
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    Target::new(TargetKind::Stdout),  // 输出到终端
                    Target::new(TargetKind::LogDir { file_name: None }),  // 输出到文件 (AppData/logs)
                    Target::new(TargetKind::Webview), // 输出到前端控制台
                ])
                .level(log::LevelFilter::Debug) // 设置全局日志级别
                .build(),
        )

        .setup(|app| {
            // database
            let conn = db::init(app.app_handle())?;
            app.manage(DbState(Mutex::new(conn)));

            let show = MenuItem::with_id(app, "show", "打开", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;

            TrayIconBuilder::new()
                .menu(&menu)
                // menu click
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                // tray click
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();

                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?; // 必须 build
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        // function
        .invoke_handler(tauri::generate_handler![
            import_image_from_folder,
            get_categories,
            get_memes_by_category,
            update_memes_order,
            copy_image,
            move_memes_to_category,
            create_category,
            update_category_order,
            delete_category,
            migrate_memes_data,
            rename_category,
            open_log_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
