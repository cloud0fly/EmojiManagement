use crate::db::queries;
use crate::models::meme_model::ImportResult;
use crate::utils::file_utils::*;
use crate::utils::image_utils::*;
use crate::DbState;
use std::fs;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn import_image_from_folder(
    state: State<'_, DbState>,
    folder_path: String,
    target_cat_id: i32,
) -> Result<ImportResult, String> {
    let mut conn = state.0.lock().unwrap();
    let root_path = PathBuf::from(&folder_path);

    let mut all_images: Vec<PathBuf> = Vec::new();
    scan_recursive(&root_path, &mut all_images);

    let original_dir = PathBuf::from("./data/original");
    fs::create_dir_all(&original_dir).map_err(|e| e.to_string())?;

    // 开启事务
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let mut new_count = 0;
    // let mut mismatch_count = 0;

    {
        let mut stmt = tx
            .prepare(queries::INSERT_MEME_IF_NOT_EXISTS)
            .map_err(|e| e.to_string())?;

        for img_path in &all_images {
            // 2. 检查后缀真实性
            let (is_match, real_ext) = check_image_type(&img_path);
            if !is_match {
                // mismatch_count += 1;
                println!("格式不符: {:?} 实际是 {}", img_path, real_ext);
            }
            let md5_hash = calculate_md5(img_path)?;
            let is_gif = real_ext == "gif";

            // 4. 构建最终路径
            let file_name = format!("{}.{}", md5_hash, real_ext);
            let final_path = original_dir.join(file_name);
            let final_path_str = final_path.to_string_lossy().to_string();

            // 5. 执行插入
            let changes = stmt
                .execute(rusqlite::params![
                    final_path_str,
                    target_cat_id,
                    is_gif,
                    md5_hash
                ])
                .map_err(|e| {
                    println!("数据库执行报错: {}", e);
                    e.to_string()
                })?;

            println!("MD5: {}, Changes: {}", md5_hash, changes); // 确认这里是否有 changes > 0

            if changes > 0 {
                if !final_path.exists() {
                    fs::copy(img_path, &final_path).map_err(|e| e.to_string())?;
                }
                new_count += 1;
            } else {
                // 为什么没进入？
                println!("跳过重复项: {}", md5_hash);
            }
        }
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(ImportResult {
        total: all_images.len() as u32,
        new: new_count,
    })
}

#[tauri::command]
pub async fn update_memes_order(
    state: tauri::State<'_, DbState>,
    ids: Vec<i32>,
) -> Result<(), String> {
    let mut conn = state.0.lock().unwrap();

    // 开启事务
    let tx = conn
        .transaction()
        .map_err(|e| format!("开启事务失败: {}", e))?;

    {
        let mut stmt = tx
            .prepare(queries::UPDATE_MEME_ORDER)
            .map_err(|e| e.to_string())?;

        for (index, id) in ids.iter().enumerate() {
            stmt.execute(rusqlite::params![index as i32, id])
                .map_err(|e| format!("更新 ID {} 失败: {}", id, e))?;
        }
    }

    tx.commit().map_err(|e| format!("提交事务失败: {}", e))?;

    Ok(())
}
