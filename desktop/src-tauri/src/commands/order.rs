use crate::db;

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
