use crate::db::queries;
use crate::DbState;
use arboard::Clipboard;
use image::io::Reader as ImageReader;
use infer;
use md5;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::State;
use serde::{Serialize, Deserialize};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[tauri::command]
pub fn copy_image(path: String) -> Result<(), String> {
    let ext = Path::new(&path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    if ext == "gif" {
        let script = format!("Set-Clipboard -Path '{}'", path);
        Command::new("powershell")
            .args(["-Command", &script])
            .creation_flags(0x08000000)
            .status()
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        copy_as_image(&path)
    }
}

fn copy_as_image(path: &str) -> Result<(), String> {
    let img = ImageReader::open(path)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?
        .to_rgba8();
    let (width, height) = img.dimensions();
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    clipboard
        .set_image(arboard::ImageData {
            width: width as usize,
            height: height as usize,
            bytes: std::borrow::Cow::Owned(img.into_raw()),
        })
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct MemeItem {
    pub id: i32,
    pub file_path: String,
    pub is_gif: bool,
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
pub async fn update_memes_order(
    state: tauri::State<'_, DbState>,
    ids: Vec<i32>
) -> Result<(), String> {
    let mut conn = state.0.lock().unwrap();
    
    // 开启事务：这是保证性能的关键！
    let tx = conn.transaction().map_err(|e| format!("开启事务失败: {}", e))?;

    {
        // 在事务作用域内准备 SQL 语句
        let mut stmt = tx.prepare(queries::UPDATE_MEME_ORDER)
            .map_err(|e| e.to_string())?;

        // 遍历前端传来的 ID 数组，数组下标即为新的排序索引
        for (index, id) in ids.iter().enumerate() {
            stmt.execute(rusqlite::params![index as i32, id])
                .map_err(|e| format!("更新 ID {} 失败: {}", id, e))?;
        }
    } // stmt 在这里被 drop，以便后续 commit

    tx.commit().map_err(|e| format!("提交事务失败: {}", e))?;
    
    Ok(())
}

fn check_image_type(path: &Path) -> (bool, String) {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    let info = infer::get_from_path(path).unwrap_or(None);
    match info {
        Some(t) => {
            let real_ext = t.extension();
            let is_match =
                if (ext == "jpg" || ext == "jpeg") && (real_ext == "jpg" || real_ext == "jpeg") {
                    true
                } else {
                    ext == real_ext
                };
            (is_match, real_ext.to_string())
        }
        None => (false, ext),
    }
}

#[derive(serde::Serialize)]
pub struct ImportResult {
    pub total: u32,
    pub new: u32,
}

fn calculate_md5(path: &Path) -> Result<String, String> {
    let content =
        fs::read(path).map_err(|e| format!("读取文件失败 ({}): {}", path.display(), e))?;
    let digest = md5::compute(content);
    Ok(format!("{:x}", digest)) // 转为十六进制字符串
}

fn is_supported_image(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            let ext = ext.to_lowercase();
            ext == "jpg" || ext == "jpeg" || ext == "png" || ext == "gif" || ext == "webp"
        })
        .unwrap_or(false)
}

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

    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let mut new_count = 0;
    let mut mismatch_count = 0;

    {
        let mut stmt = tx
            .prepare(queries::INSERT_MEME_IF_NOT_EXISTS)
            .map_err(|e| e.to_string())?;

        for img_path in &all_images {
            // 1. 检查后缀真实性
            let (is_match, real_ext) = check_image_type(&img_path);
            if !is_match {
                mismatch_count += 1;
                println!("检测到格式不符: {:?} 实际是 {}", img_path, real_ext);
            }

            // 2. 唯一性检查
            let md5_hash = calculate_md5(&img_path)?;
            let path_str = img_path.to_string_lossy().to_string();
            let is_gif = real_ext == "gif";

            // 3. 执行插入
            let changes = stmt
                .execute(rusqlite::params![path_str, target_cat_id, is_gif, md5_hash])
                .map_err(|e| e.to_string())?;

            if changes > 0 {
                new_count += 1;
            }
        }
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(ImportResult {
        total: all_images.len() as u32,
        new: new_count,
    })
}
