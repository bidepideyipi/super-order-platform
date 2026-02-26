#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod commands;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_version,
            commands::open_file,
            commands::save_file,
            commands::open_external,
            commands::sku_list,
            commands::sku_list_paginated,
            commands::sku_get,
            commands::sku_create,
            commands::sku_update,
            commands::sku_delete,
            commands::sku_search,
            commands::sku_search_paginated,
            commands::sku_search_with_category,
            commands::sku_get_image,
            commands::category_list,
            commands::customer_list,
            commands::order_list,
            commands::order_get,
            commands::order_create,
            commands::order_update,
            commands::order_delete,
            commands::get_processing_orders,
            commands::get_order_items,
            commands::search_sku_by_code,
            commands::create_order_item,
            commands::update_order_item,
            commands::delete_order_item,
            commands::financial_list,
            commands::financial_get,
            commands::financial_create,
            commands::financial_update,
            commands::financial_delete,
            commands::financial_get_balance
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
