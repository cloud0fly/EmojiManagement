use std::process::Command;
use std::path::{Path};
use crate::utils::image_utils::copy_as_image;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

// 复制图片至剪贴板
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