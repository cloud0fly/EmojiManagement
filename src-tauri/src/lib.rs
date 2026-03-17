// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use arboard::Clipboard;
use image::io::Reader as ImageReader;
use std::path::Path;
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

// 处理图片: 复制至剪贴板
#[tauri::command]
fn copy_image(path: String) -> Result<(), String> {
    let ext = Path::new(&path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    if ext == "gif" {
        // gif复制方案：通过 PowerShell 将文件作为文件对象放入剪贴板
        let script = format!("Set-Clipboard -Path '{}'", path);

        Command::new("powershell")
            .arg("-Command")
            .arg(&script)
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![copy_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
