use tauri::State;
use crate::DbState;
use crate::db::queries;

#[tauri::command]
pub fn get_categories(state: State<'_, DbState>) -> Result<Vec<serde_json::Value>, String> {
    let conn = state.0.lock().unwrap();
    let mut stmt = conn.prepare(queries::GET_ALL_CATEGORIES).map_err(|e| e.to_string())?;
    
    let rows = stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i32>(0)?,
            "name": row.get::<_, String>(1)?,
            "orderIndex": row.get::<_, i32>(2)?,
        }))
    }).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}