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
