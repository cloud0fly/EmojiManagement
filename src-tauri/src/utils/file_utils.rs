use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use crate::utils::image_utils::is_supported_image;

pub fn move_to_original(temp: &Path, dest: &Path) -> Result<(), String> {
    // 跨磁盘安全
    std::fs::copy(temp, dest).map_err(|e| e.to_string())?;
    std::fs::remove_file(temp).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn calculate_md5(path: &Path) -> Result<String, String> {
    let content =
        fs::read(path).map_err(|e| format!("读取文件失败 ({}): {}", path.display(), e))?;
    let digest = md5::compute(content);
    Ok(format!("{:x}", digest)) // 转为十六进制字符串
}

pub fn scan_recursive(path: &Path, result: &mut Vec<PathBuf>) {
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