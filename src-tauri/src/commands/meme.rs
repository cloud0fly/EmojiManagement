// meme.rs M001
use crate::db::queries::*;
use crate::models::meme_model::ImportResult;
use crate::models::meme_model::MemeItem;
use crate::utils::file_utils::*;
use crate::utils::image_utils::*;
use crate::DbState;
use log::{debug, error, info, warn};
use tauri::Manager;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::AppHandle;
use tauri::State;

// 导入图片
#[tauri::command]
pub async fn import_image_from_folder(
    app: AppHandle,
    state: State<'_, DbState>,
    folder_path: String,
    target_cat_id: i32,
) -> Result<ImportResult, String> {
    info!(
        "开始导入图片任务，源路径: {}, 目标分类: {}",
        folder_path, target_cat_id
    );

    let mut all_images: Vec<PathBuf> = Vec::new();
    let mut new_count = 0;
    let relative_folder = "original";

    // 开启事务
    let mut conn = state.0.lock().unwrap();
    let tx = conn.transaction().map_err(|e| {
        error!("开启数据库事务失败: {}", e);
        e.to_string()
    })?;

    // 读取配置文件中的保存路径
    let base_path = get_meme_path(&app);
    let root_path = PathBuf::from(&folder_path);

    fn scan_recursive(path: &Path, result: &mut Vec<PathBuf>) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    scan_recursive(&p, result);
                } else if is_supported_image(&p) {
                    result.push(p);
                }
            }
        }
    }
    scan_recursive(&root_path, &mut all_images);
    info!("扫描完成，找到潜在图片 {} 张", all_images.len());

    // 创建文件夹
    let original_dir = base_path.join(relative_folder);
    if !original_dir.exists() {
        debug!("创建存储目录: {:?}", original_dir);
        fs::create_dir_all(&original_dir).map_err(|e| {
            error!("创建文件夹失败: {:?}, 错误: {}", original_dir, e);
            e.to_string()
        })?;
    }

    {
        let mut stmt = tx
            .prepare(INSERT_MEME_IF_NOT_EXISTS)
            .map_err(|e| e.to_string())?;

        for img_path in &all_images {
            // 检查后缀真实性
            let (is_match, real_ext) = check_image_type(&img_path);
            if !is_match {
                warn!("文件格式校验未通过: {:?}, 实际后缀: {}", img_path, real_ext);
            }

            let md5_hash = calculate_md5_stream(img_path)?;
            let is_gif = real_ext == "gif";

            // 构建文件名及路径
            let file_name = format!("{}.{}", md5_hash, real_ext);
            let final_path = original_dir.join(&file_name);
            let relative_path = format!("{}/{}", relative_folder, file_name);

            let changes = stmt
                .execute(rusqlite::params![
                    relative_path,
                    target_cat_id,
                    is_gif,
                    md5_hash
                ])
                .map_err(|e| {
                    warn!("数据库插入跳过 (可能是重复 MD5): {}, 错误: {}", md5_hash, e);
                    e.to_string()
                })?;
            if changes > 0 {
                if !final_path.exists() {
                    if let Err(e) = fs::copy(img_path, &final_path) {
                        error!(
                            "物理文件拷贝失败: 从 {:?} 到 {:?}, 错误: {}",
                            img_path, final_path, e
                        );
                        return Err(format!("文件拷贝失败: {}", e));
                    }
                }
                new_count += 1;
            }
        }
    }

    tx.commit().map_err(|e| {
        error!("数据库事务提交失败: {}", e);
        e.to_string()
    })?;

    info!(
        "导入任务完成: 总计 {}, 新增 {}",
        all_images.len(),
        new_count
    );
    Ok(ImportResult {
        total: all_images.len() as u32,
        new: new_count,
    })
}

// 按分组获取图片
#[tauri::command]
pub async fn get_memes_by_category(
    app: tauri::AppHandle,
    state: State<'_, DbState>,
    cat_id: i32,
) -> Result<Vec<MemeItem>, String> {
    let conn = state.0.lock().unwrap();
    let base_path = get_meme_path(&app);
    let mut stmt = conn.prepare(GET_MEMES_BY_CATEGORY).map_err(|e| e.to_string())?;

    let meme_iter = stmt
        .query_map(rusqlite::params![cat_id], |row| {
            let relative_path: String = row.get(1)?;
            let full_path = base_path.join(&relative_path).to_string_lossy().to_string();
            Ok(MemeItem {
                id: row.get(0)?,
                file_path: full_path,
                is_gif: row.get(2)?,
            })
        })
        .map_err(|e| {
            error!("查询分类 {} 图片失败: {}", cat_id, e);
            e.to_string()
        })?;

    let mut memes = Vec::new();
    for meme in meme_iter {
        memes.push(meme.map_err(|e| e.to_string())?);
    }

    Ok(memes)
}

// 重排序(用于拖拽排序逻辑)
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
        let mut stmt = tx.prepare(UPDATE_MEME_ORDER).map_err(|e| e.to_string())?;

        for (index, id) in ids.iter().enumerate() {
            stmt.execute(rusqlite::params![index as i32, id])
                .map_err(|e| format!("更新 ID {} 失败: {}", id, e))?;
        }
    }

    tx.commit().map_err(|e| format!("提交事务失败: {}", e))?;

    Ok(())
}

// 迁移图片数据
#[tauri::command]
pub async fn migrate_memes_data(app: AppHandle, new_path: String) -> Result<(), String> {
    let old_path = get_meme_path(&app);
    let target_path = PathBuf::from(&new_path);

    info!(
        "启动存储路径迁移: 从 {:?} 搬迁至 {:?}",
        old_path, target_path
    );

    if old_path == target_path {
        warn!("新旧路径相同，跳过迁移");
        return Ok(());
    }

    if !target_path.exists() {
        fs::create_dir_all(&target_path).map_err(|e| {
            error!("创建目标目录失败: {}", e);
            e.to_string()
        })?;
    }

    if old_path.exists() && old_path.is_dir() {
        let entries = fs::read_dir(&old_path).map_err(|e| {
            error!("读取旧目录失败: {}", e);
            e.to_string()
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_name = entry.file_name();
            let dest_file = target_path.join(file_name);

            if let Err(e) = fs::rename(entry.path(), &dest_file) {
                warn!("直接重命名失败 (可能是跨盘符): {}, 尝试复制+删除方式", e);
                fs::copy(entry.path(), &dest_file).map_err(|e| e.to_string())?;
                fs::remove_file(entry.path()).ok();
            }
        }

        // 删除旧的空文件夹 暂不使用
        // fs::remove_dir(old_path).ok();
        info!("所有物理文件迁移完成");
    } else {
        warn!("旧目录不存在，无需移动物理文件");
    }

    Ok(())
}

#[tauri::command]
pub fn open_log_dir(app: tauri::AppHandle) {
    let log_dir = app.path().app_log_dir().unwrap();
    if log_dir.exists() {
        // 使用系统的资源管理器打开
        #[cfg(target_os = "windows")]
        std::process::Command::new("explorer").arg(log_dir).spawn().ok();
        
        #[cfg(target_os = "macos")]
        std::process::Command::new("open").arg(log_dir).spawn().ok();
    }
}
