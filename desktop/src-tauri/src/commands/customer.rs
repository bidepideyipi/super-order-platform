use crate::db;

#[tauri::command]
pub fn customer_list() -> Result<Vec<db::Customer>, String> {
    println!("customer_list called");
    db::get_customers().map_err(|e| {
        println!("customer_list error: {}", e);
        e.to_string()
    })
}
