use tauri::State;
use crate::DbState;
use crate::db::queries;
use crate::models::meme_model::MemeItem;

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

#[tauri::command]
pub async fn get_memes_by_category(
    state: State<'_, DbState>,
    cat_id: i32,
) -> Result<Vec<MemeItem>, String> {
    let conn = state.0.lock().unwrap();
    let mut stmt = conn.prepare(queries::GET_MEMES_BY_CAT)
        .map_err(|e| e.to_string())?;

    let meme_iter = stmt.query_map(rusqlite::params![cat_id], |row| {
        Ok(MemeItem {
            id: row.get(0)?,
            file_path: row.get(1)?,
            is_gif: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut memes = Vec::new();
    for meme in meme_iter {
        memes.push(meme.map_err(|e| e.to_string())?);
    }

    Ok(memes)
}

#[tauri::command]
pub fn move_memes_to_category(
    state: State<'_, DbState>,
    meme_ids: Vec<i32>,
    target_cat_id: i32,
) -> Result<(), String> {
    let mut conn = state.0.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    {
        let mut stmt = tx.prepare(
            queries::UPDATE_MEME_CATEGORY
        ).map_err(|e| e.to_string())?;

        for id in meme_ids {
            stmt.execute(rusqlite::params![target_cat_id, id])
                .map_err(|e| e.to_string())?;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn create_category(
    state: State<'_, DbState>,
    name: String,
) -> Result<i32, String> {
    let conn = state.0.lock().unwrap();

    conn.execute(
        queries::CREATE_CATEGORY,
        rusqlite::params![name],
    ).map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid() as i32)
}

#[tauri::command]
pub fn update_category_order(
    state: State<'_, DbState>,
    ids: Vec<i32>,
) -> Result<(), String> {
    let mut conn = state.0.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    {
        let mut stmt = tx.prepare(
            queries::UPDATE_CATEGORY_ORDER
        ).map_err(|e| e.to_string())?;

        for (index, id) in ids.iter().enumerate() {
            stmt.execute(rusqlite::params![index as i32, id])
                .map_err(|e| e.to_string())?;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_category(
    state: State<'_, DbState>,
    cat_id: i32,
) -> Result<(), String> {
    let conn = state.0.lock().unwrap();

    conn.execute(
        queries::DELETE_CATEGORY,
        rusqlite::params![cat_id],
    ).map_err(|e| e.to_string())?;

    Ok(())
}