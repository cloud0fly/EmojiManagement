pub mod queries;
use rusqlite::{Connection, Result};
use std::fs;
use tauri::AppHandle;
use tauri::Manager;

pub fn init(app_handle: &AppHandle) -> Result<Connection, String> {
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let db_path = app_dir.join("memes.db");

    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

    // 使用常量初始化
    conn.execute(queries::INIT_CATEGORIES_TABLE, []).map_err(|e| e.to_string())?;
    conn.execute(queries::INIT_MEMES_TABLE, []).map_err(|e| e.to_string())?;
    conn.execute(queries::INSERT_DEFAULT_CATEGORY, []).map_err(|e| e.to_string())?;

    Ok(conn)
}