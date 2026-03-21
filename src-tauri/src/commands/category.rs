use crate::db::queries::*;
use crate::DbState;
use log::{debug, error, info, warn};
use tauri::State;

// 获取所有分类列表
#[tauri::command]
pub fn get_categories(state: State<'_, DbState>) -> Result<Vec<serde_json::Value>, String> {
    let conn = state.0.lock().unwrap();
    let mut stmt = conn.prepare(GET_ALL_CATEGORIES).map_err(|e| {
        error!("准备获取分类 SQL 失败: {}", e);
        e.to_string()
    })?;

    let rows = stmt
        .query_map([], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, i32>(0)?,
                "name": row.get::<_, String>(1)?,
                "orderIndex": row.get::<_, i32>(2)?,
            }))
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

// 批量移动表情包到指定分类
#[tauri::command]
pub fn move_memes_to_category(
    state: State<'_, DbState>,
    meme_ids: Vec<i32>,
    target_cat_id: i32,
) -> Result<(), String> {
    info!(
        "开始批量移动表情包: 数量={}, 目标分类ID={}",
        meme_ids.len(),
        target_cat_id
    );

    let mut conn = state.0.lock().unwrap();
    let tx = conn.transaction().map_err(|e| {
        error!("开启移动事务失败: {}", e);
        e.to_string()
    })?;

    {
        let mut stmt = tx
            .prepare(UPDATE_MEME_CATEGORY)
            .map_err(|e| e.to_string())?;

        for id in meme_ids {
            stmt.execute(rusqlite::params![target_cat_id, id])
                .map_err(|e| {
                    error!("更新表情包分类失败 (ID: {}): {}", id, e);
                    e.to_string()
                })?;
        }
    }

    tx.commit().map_err(|e| {
        error!("提交移动事务失败: {}", e);
        e.to_string()
    })?;
    info!("批量移动操作完成");
    Ok(())
}

// 创建新分类
#[tauri::command]
pub fn create_category(state: State<'_, DbState>, name: String) -> Result<i32, String> {
    info!("正在创建新分类: '{}'", name);

    let conn = state.0.lock().unwrap();
    conn.execute(CREATE_CATEGORY, rusqlite::params![name])
        .map_err(|e| {
            error!("创建分类失败: {}, 错误: {}", name, e);
            e.to_string()
        })?;

    Ok(conn.last_insert_rowid() as i32)
}

// 更新分类排序
#[tauri::command]
pub fn update_category_order(state: State<'_, DbState>, ids: Vec<i32>) -> Result<(), String> {
    let mut conn = state.0.lock().unwrap();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    {
        let mut stmt = tx
            .prepare(UPDATE_CATEGORY_ORDER)
            .map_err(|e| e.to_string())?;

        for (index, id) in ids.iter().enumerate() {
            stmt.execute(rusqlite::params![index as i32, id]).map_err(|e| {
                error!("更新分类顺序失败 (ID: {}): {}", id, e);
                e.to_string()
            })?;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

/// 重命名分类
#[tauri::command]
pub fn rename_category(
    state: State<'_, DbState>,
    cat_id: i32,
    new_name: String,
) -> Result<(), String> {
    let name = new_name.trim();
    if name.is_empty() {
        warn!("分类重命名失败: 名字不能为空");
        return Err("分类名称不能为空".into());
    }

    info!("正在将分类 ID {} 重命名为: '{}'", cat_id, name);
    let conn = state.0.lock().unwrap();

    let affected = conn.execute(
        UPDATE_CATEGORY_NAME,
        rusqlite::params![name, cat_id],
    ).map_err(|e| {
        error!("数据库更新分类名失败: {}", e);
        e.to_string()
    })?;

    if affected == 0 {
        warn!("未发现 ID 为 {} 的分类，重命名未生效", cat_id);
    } else {
        debug!("分类重命名成功");
    }

    Ok(())
}

// 删除分类并将所属表情包移入默认分类
#[tauri::command]
pub fn delete_category(state: State<'_, DbState>, cat_id: i32) -> Result<(), String> {
    info!("正在请求删除分类，ID: {}", cat_id);

    let default_cat_id = 1;
    if cat_id == default_cat_id {
        warn!("尝试删除默认分类，操作已被拦截");
        return Err("禁止删除默认分类".into());
    }

    let mut conn = state.0.lock().unwrap();
    conn.execute("PRAGMA foreign_keys = ON;", []).ok();
    let tx = conn.transaction().map_err(|e| {
        error!("开启删除分类事务失败: {}", e);
        e.to_string()
    })?;

    {
        debug!("正在将分类 {} 下的表情包迁移至分类 {}", cat_id, default_cat_id);
        tx.execute(
            MIGRATE_MEMES,
            rusqlite::params![default_cat_id, cat_id],
        ).map_err(|e| {
            error!("迁移表情包失败: {}", e);
            e.to_string()
        })?;

        tx.execute(
            DELETE_CATEGORY,
            rusqlite::params![cat_id],
        ).map_err(|e| {
            error!("执行删除分类 SQL 失败: {}", e);
            e.to_string()
        })?;
    }

    tx.commit().map_err(|e| {
        error!("提交删除分类事务失败: {}", e);
        e.to_string()
    })?;

    info!("分类 ID {} 已成功删除", cat_id);
    Ok(())
}
