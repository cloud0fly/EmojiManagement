use arboard::Clipboard;
use image::ImageReader;
use log::{error, info, warn};
use std::path::Path;

// 复制图片至剪贴板
pub fn copy_as_image(path: &str) -> Result<(), String> {
    let img_reader = ImageReader::open(path).map_err(|e| {
        error!("打开图片文件失败: {}, 错误: {}", path, e);
        e.to_string()
    })?;

    let decoded_img = img_reader.decode().map_err(|e| {
        error!("解码图片失败: {}, 错误: {}", path, e);
        e.to_string()
    })?;

    let rgba_img = decoded_img.to_rgba8();
    let (width, height) = rgba_img.dimensions();

    let mut clipboard = Clipboard::new().map_err(|e| {
        error!("初始化系统剪贴板失败: {}", e);
        e.to_string()
    })?;

    clipboard
        .set_image(arboard::ImageData {
            width: width as usize,
            height: height as usize,
            bytes: std::borrow::Cow::Owned(rgba_img.into_raw()),
        })
        .map_err(|e| {
            error!("写入剪贴板失败: {}", e);
            e.to_string()
        })?;
    info!("图片已成功复制到剪贴板: {} ({}x{})", path, width, height);
    Ok(())
}

// 检查类型
pub fn check_image_type(path: &Path) -> (bool, String) {
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
            if !is_match {
                warn!(
                    "检测到文件后缀名不匹配: {:?}, 声明为 {}, 实际为 {}",
                    path, ext, real_ext
                );
            }
            (is_match, real_ext.to_string())
        }
        None => {
            warn!(
                "无法识别文件真实类型: {:?}, 回退至声明的后缀: {}",
                path, ext
            );
            (false, ext)
        }
    }
}

// 判断是否为支持的类型
pub fn is_supported_image(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            let ext = ext.to_lowercase();
            ext == "jpg" || ext == "jpeg" || ext == "png" || ext == "gif" || ext == "webp"
        })
        .unwrap_or(false)
}
