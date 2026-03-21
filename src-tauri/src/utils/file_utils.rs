use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;
use log::{error, warn};

// 计算md5
pub fn calculate_md5_stream(path: &Path) -> Result<String, String> {
    let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut context = md5::Context::new();
    let mut buffer = [0; 1024];
    loop {
        let n = file.read(&mut buffer).map_err(|e| e.to_string())?;
        if n == 0 { break; }
        context.consume(&buffer[..n]);
    }
    Ok(format!("{:x}", context.finalize()))
}

// 获取配置中的存储路径
pub fn get_meme_path(app: &AppHandle) -> PathBuf {
    let stores = app.store("settings.json");
    
    match stores {
        Ok(store) => {
            if let Some(value) = store.get("memePath") {
                if let Some(path_str) = value.as_str() {
                    if !path_str.is_empty() {
                        let custom_path = PathBuf::from(path_str);
                        return custom_path;
                    }
                }
            }
   
            let default_path = app.path().app_data_dir().expect("获取 AppData 目录失败").join("memes");
            warn!("配置中 memePath 为空，将使用默认路径: {:?}", default_path);
            default_path
        },

        Err(e) => {
            let default_path = app.path().app_data_dir().expect("获取 AppData 目录失败").join("memes");
            error!("加载 settings.json 失败: {}, 回退至默认路径: {:?}", e, default_path);
            default_path
        },
    }
}