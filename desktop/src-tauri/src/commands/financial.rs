use crate::db::{self, FinancialTransaction};

#[tauri::command]
pub fn financial_list() -> Result<Vec<FinancialTransaction>, String> {
    println!("financial_list called");
    db::get_financial_transactions().map_err(|e| {
        println!("financial_list error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn financial_get(id: String) -> Result<Option<FinancialTransaction>, String> {
    println!("financial_get called with id: {}", id);
    let id_parsed = id.parse::<i64>().map_err(|e| e.to_string())?;
    db::get_financial_transaction(id_parsed).map_err(|e| {
        println!("financial_get error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn financial_create(data: FinancialTransaction) -> Result<FinancialTransaction, String> {
    println!("financial_create called");
    db::create_financial_transaction(data).map_err(|e| {
        println!("financial_create error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn financial_update(id: String, data: FinancialTransaction) -> Result<FinancialTransaction, String> {
    println!("financial_update called with id: {}", id);
    let id_parsed = id.parse::<i64>().map_err(|e| e.to_string())?;
    db::update_financial_transaction(id_parsed, data).map_err(|e| {
        println!("financial_update error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn financial_delete(id: String) -> Result<(), String> {
    println!("financial_delete called with id: {}", id);
    let id_parsed = id.parse::<i64>().map_err(|e| e.to_string())?;
    db::delete_financial_transaction(id_parsed).map_err(|e| {
        println!("financial_delete error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn financial_get_balance() -> Result<f64, String> {
    println!("financial_get_balance called");
    db::get_current_balance().map_err(|e| {
        println!("financial_get_balance error: {}", e);
        e.to_string()
    })
}
