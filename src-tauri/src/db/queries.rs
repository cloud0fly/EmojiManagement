// src-tauri/src/db/queries.rs

pub const INIT_CATEGORIES_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS categories (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        order_index INTEGER DEFAULT 0
    );
";

pub const INIT_MEMES_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS memes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        file_path TEXT UNIQUE NOT NULL,
        category_id INTEGER,
        order_index INTEGER DEFAULT 0,
        md5 TEXT UNIQUE,
        is_gif BOOLEAN,
        FOREIGN KEY(category_id) REFERENCES categories(id)
    );
";

pub const INSERT_DEFAULT_CATEGORY: &str = "
    INSERT OR IGNORE INTO categories (id, name) VALUES (1, 'default')
";

pub const GET_ALL_CATEGORIES: &str = "
    SELECT id, name, order_index FROM categories ORDER BY order_index ASC;
";

pub const UPDATE_MEME_CATEGORY: &str = "
    UPDATE memes SET category_id = ? WHERE id = ?;
";

pub const CREATE_CATEGORY: &str = "INSERT INTO categories (name, order_index) VALUES (?, 0)";

pub const INSERT_MEME_IF_NOT_EXISTS: &str = "
    INSERT INTO memes (file_path, category_id, is_gif, md5)
    SELECT ?1, ?2, ?3, ?4
    WHERE NOT EXISTS (
        SELECT 1 FROM memes WHERE md5 = ?4
    );
";

pub const GET_MEMES_BY_CAT: &str = "
    SELECT id, file_path, is_gif, category_id, order_index 
    FROM memes 
    WHERE category_id = ? 
    ORDER BY order_index ASC
";

pub const UPDATE_MEME_ORDER: &str = "UPDATE memes SET order_index = ? WHERE id = ?";

pub const UPDATE_CATEGORY_ORDER: &str = "UPDATE categories SET order_index = ? WHERE id = ?";

pub const DELETE_CATEGORY: &str = "DELETE FROM categories WHERE id = ?";