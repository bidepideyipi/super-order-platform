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
    db::get_all_skus().map_err(|e| {
        println!("sku_list error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn sku_get(_id: String) -> Result<Option<db::SKU>, String> {
    println!("sku_get called with id: {}", _id);
    Ok(None)
}

#[tauri::command]
pub fn sku_create(_data: db::SKU) -> Result<db::SKU, String> {
    println!("sku_create called");
    Ok(_data)
}

#[tauri::command]
pub fn sku_update(_id: String, _data: db::SKU) -> Result<db::SKU, String> {
    println!("sku_update called with id: {}", _id);
    Ok(_data)
}

#[tauri::command]
pub fn sku_delete(_id: String) -> Result<(), String> {
    println!("sku_delete called with id: {}", _id);
    Ok(())
}

#[tauri::command]
pub fn sku_search(_keyword: String) -> Result<Vec<db::SKU>, String> {
    println!("sku_search called with keyword: {}", _keyword);
    Ok(vec![])
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
