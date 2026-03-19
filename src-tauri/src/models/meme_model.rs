use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MemeItem {
    pub id: i32, // id
    pub file_path: String, // 文件路径
    pub is_gif: bool, // gif标识
}

#[derive(serde::Serialize)]
pub struct ImportResult {
    pub total: u32,
    pub new: u32,
}