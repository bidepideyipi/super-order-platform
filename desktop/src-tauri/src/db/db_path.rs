use std::path::PathBuf;
use std::fs;

pub fn get_db_path() -> PathBuf {
    let data_dir = get_data_dir();
    data_dir.join("super_order.db")
}

pub fn get_images_dir() -> PathBuf {
    let data_dir = get_data_dir();
    let images_dir = data_dir.join("images").join("sku");
    
    if !images_dir.exists() {
        fs::create_dir_all(&images_dir).unwrap_or_else(|e| {
            println!("Failed to create images directory: {}", e);
        });
    }
    
    images_dir
}

fn get_data_dir() -> PathBuf {
    let base_dir = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    
    let data_dir = base_dir.parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
        .join("data");
    
    println!("Data directory: {:?}", data_dir);
    
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).unwrap_or_else(|e| {
            println!("Failed to create data directory: {}", e);
        });
    }
    
    data_dir
}
