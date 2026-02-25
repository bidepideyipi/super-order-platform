use crate::db;

#[tauri::command]
pub fn category_list() -> Result<Vec<db::Category>, String> {
    println!("category_list called");
    db::get_categories().map_err(|e| {
        println!("category_list error: {}", e);
        e.to_string()
    })
}
