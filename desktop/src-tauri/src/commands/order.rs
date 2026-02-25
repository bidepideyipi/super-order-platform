use crate::db::{self, Order};

#[tauri::command]
pub fn order_list() -> Result<Vec<Order>, String> {
    println!("order_list called");
    db::get_orders().map_err(|e| {
        println!("order_list error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn order_get(id: String) -> Result<Option<Order>, String> {
    println!("order_get called with id: {}", id);
    let id_parsed = id.parse::<i64>().map_err(|e| e.to_string())?;
    db::get_order(id_parsed).map_err(|e| {
        println!("order_get error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn order_create(data: Order) -> Result<Order, String> {
    println!("order_create called");
    db::create_order(data).map_err(|e| {
        println!("order_create error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn order_update(id: String, data: Order) -> Result<Order, String> {
    println!("order_update called with id: {}", id);
    let id_parsed = id.parse::<i64>().map_err(|e| e.to_string())?;
    db::update_order(id_parsed, data).map_err(|e| {
        println!("order_update error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn order_delete(id: String) -> Result<(), String> {
    println!("order_delete called with id: {}", id);
    let id_parsed = id.parse::<i64>().map_err(|e| e.to_string())?;
    db::delete_order(id_parsed).map_err(|e| {
        println!("order_delete error: {}", e);
        e.to_string()
    })
}
