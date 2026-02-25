use crate::db;
use base64::Engine;

#[tauri::command]
pub fn sku_list() -> Result<Vec<db::SKU>, String> {
    println!("sku_list called");
    match db::get_all_skus() {
        Ok(skus) => {
            println!("sku_list success: {} skus returned", skus.len());
            Ok(skus)
        }
        Err(e) => {
            println!("sku_list error: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub fn sku_list_paginated(page: usize, page_size: usize) -> Result<db::PaginatedResult<db::SKU>, String> {
    println!("sku_list_paginated called: page={}, page_size={}", page, page_size);
    match db::get_skus_paginated(page, page_size) {
        Ok(result) => {
            println!("sku_list_paginated success: {} items, total: {}", result.data.len(), result.total);
            Ok(result)
        }
        Err(e) => {
            println!("sku_list_paginated error: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub fn sku_get(_id: String) -> Result<Option<db::SKU>, String> {
    println!("sku_get called with id: {}", _id);
    Ok(None)
}

#[tauri::command]
pub fn sku_create(mut data: db::SKU, image_base64: Option<String>) -> Result<db::SKU, String> {
    println!("sku_create called");
    data.is_deleted = false;
    match db::create_sku(data, image_base64) {
        Ok(sku) => {
            println!("sku_create success: {} with sku_code {:?}", sku.name, sku.sku_code);
            Ok(sku)
        }
        Err(e) => {
            println!("sku_create error: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub fn sku_update(id: String, mut data: db::SKU, image_base64: Option<String>) -> Result<db::SKU, String> {
    println!("sku_update called with id: {}", id);
    data.is_deleted = false;
    match db::update_sku(id.parse::<i64>().unwrap_or(0), data, image_base64) {
        Ok(sku) => {
            println!("sku_update success: {} with sku_code {:?}", sku.name, sku.sku_code);
            Ok(sku)
        }
        Err(e) => {
            println!("sku_update error: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub fn sku_delete(_id: String) -> Result<(), String> {
    println!("sku_delete called with id: {}", _id);
    match db::delete_sku(_id.parse::<i64>().unwrap_or(0)) {
        Ok(_) => {
            println!("sku_delete success");
            Ok(())
        }
        Err(e) => {
            println!("sku_delete error: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub fn sku_search(keyword: String) -> Result<Vec<db::SKU>, String> {
    println!("sku_search called with keyword: {}", keyword);
    db::search_skus(&keyword).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn sku_search_paginated(keyword: String, page: usize, page_size: usize) -> Result<db::PaginatedResult<db::SKU>, String> {
    println!("sku_search_paginated called with keyword: {}, page: {}, page_size: {}", keyword, page, page_size);
    db::search_skus_paginated(&keyword, page, page_size).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn sku_search_with_category(keyword: String, category_id: Option<String>) -> Result<Vec<db::SKU>, String> {
    println!("sku_search_with_category called with keyword: {}, category_id: {:?}", keyword, category_id);
    db::search_skus_by_keyword_and_category(&keyword, category_id.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn sku_get_image(sku_code: String) -> Result<Option<String>, String> {
    println!("sku_get_image called for sku_code: {}", sku_code);
    
    let images_dir = db::get_images_dir();
    let image_path = images_dir.join(&sku_code);
    
    let extensions = vec!["jpeg", "jpg", "png"];
    for ext in extensions {
        let path_with_ext = image_path.with_extension(ext);
        if path_with_ext.exists() {
            println!("Found image: {:?}", path_with_ext);
            match std::fs::read(&path_with_ext) {
                Ok(image_data) => {
                    let base64_string = base64::prelude::BASE64_STANDARD.encode(&image_data);
                    let mime_type = match ext {
                        "jpeg" | "jpg" => "image/jpeg",
                        "png" => "image/png",
                        _ => "image/jpeg",
                    };
                    return Ok(Some(format!("data:{};base64,{}", mime_type, base64_string)));
                }
                Err(e) => {
                    println!("Failed to read image: {}", e);
                    return Err(format!("Failed to read image: {}", e));
                }
            }
        }
    }
    
    println!("Image not found for sku_code: {}", sku_code);
    Ok(None)
}
