use std::path::{Path};
use arboard::Clipboard;
use image::io::Reader as ImageReader;


pub fn generate_thumbnail(original: &Path, thumb_path: &Path) {
    if let Ok(img) = image::open(original) {
        let thumb = img.thumbnail(128, 128);
        thumb.save(thumb_path).ok();
    }
}

pub fn copy_as_image(path: &str) -> Result<(), String> {
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
            (is_match, real_ext.to_string())
        }
        None => (false, ext),
    }
}

pub fn is_supported_image(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            let ext = ext.to_lowercase();
            ext == "jpg" || ext == "jpeg" || ext == "png" || ext == "gif" || ext == "webp"
        })
        .unwrap_or(false)
}

