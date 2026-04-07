use std::path::PathBuf;
use std::env;
use std::fs;

pub fn get_db_path() -> PathBuf {
    let db_path = if let Ok(path) = env::var("DATABASE_URL") {
        PathBuf::from(path)
    } else {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        current_dir.join("data/super_order.db")
    };
    
    log::info!("Database path: {:?}", db_path);
    
    if !db_path.exists() {
        log::warn!("Database file not found at {:?}", db_path);
    }
    
    db_path
}

pub fn get_images_dir() -> PathBuf {
    let db_path = get_db_path();
    let images_dir = db_path.parent()
        .map(|p| p.join("images/sku"))
        .unwrap_or_else(|| PathBuf::from("data/images/sku"));
    
    if !images_dir.exists() {
        fs::create_dir_all(&images_dir).unwrap_or_else(|e| {
            log::error!("Failed to create images directory: {}", e);
        });
    }
    
    log::info!("Images directory: {:?}", images_dir);
    images_dir
}
