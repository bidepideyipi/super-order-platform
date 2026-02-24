use crate::db;

#[tauri::command]
pub fn get_version() -> String {
    "1.0.0".to_string()
}

#[tauri::command]
pub async fn open_file() -> Result<Vec<String>, String> {
    Ok(vec![])
}

#[tauri::command]
pub async fn save_file(_default_name: String) -> Result<Option<String>, String> {
    Ok(None)
}

#[tauri::command]
pub async fn open_external(_url: String) -> Result<(), String> {
    Ok(())
}

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
pub fn sku_create(data: db::SKU, image_base64: Option<String>) -> Result<db::SKU, String> {
    println!("sku_create called");
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
pub fn sku_update(id: String, data: db::SKU, image_base64: Option<String>) -> Result<db::SKU, String> {
    println!("sku_update called with id: {}", id);
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
pub fn category_list() -> Result<Vec<db::Category>, String> {
    println!("category_list called");
    db::get_categories().map_err(|e| {
        println!("category_list error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn customer_list() -> Result<Vec<db::Customer>, String> {
    println!("customer_list called");
    db::get_customers().map_err(|e| {
        println!("customer_list error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn order_list() -> Result<Vec<db::Order>, String> {
    println!("order_list called");
    db::get_orders().map_err(|e| {
        println!("order_list error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn order_get(_id: String) -> Result<Option<db::Order>, String> {
    println!("order_get called with id: {}", _id);
    Ok(None)
}

#[tauri::command]
pub fn order_create(_data: db::Order) -> Result<db::Order, String> {
    println!("order_create called");
    Ok(_data)
}

#[tauri::command]
pub fn order_update(_id: String, _data: db::Order) -> Result<db::Order, String> {
    println!("order_update called with id: {}", _id);
    Ok(_data)
}

#[tauri::command]
pub fn order_delete(_id: String) -> Result<(), String> {
    println!("order_delete called with id: {}", _id);
    Ok(())
}
