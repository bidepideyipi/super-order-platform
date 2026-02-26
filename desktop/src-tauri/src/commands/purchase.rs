use crate::db::{self, Order, OrderItem, get_db_path, SKU};

#[tauri::command]
pub fn get_processing_orders() -> Result<Vec<Order>, String> {
    println!("get_processing_orders called");
    let conn = get_db_path();
    let rusqlite_conn = rusqlite::Connection::open(conn).map_err(|e| e.to_string())?;
    
    let mut stmt = rusqlite_conn.prepare(
        "SELECT o.id, o.order_no, o.customer_id, o.order_date, o.status, o.is_settled, 
                o.total_cost_amount, o.total_sale_amount, o.remarks
         FROM `order` o 
         WHERE o.status = 'processing'
         ORDER BY o.created_at DESC"
    ).map_err(|e| e.to_string())?;
    
    let orders = stmt.query_map([], |row| {
        Ok(Order {
            id: Some(row.get(0)?),
            order_no: row.get(1)?,
            customer_id: row.get(2)?,
            order_date: row.get(3)?,
            status: row.get(4)?,
            is_settled: row.get::<_, i32>(5)? != 0,
            total_cost_amount: row.get(6)?,
            total_sale_amount: row.get(7)?,
            remarks: row.get(8)?,
        })
    }).map_err(|e| e.to_string())?;
    
    orders.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_order_items(order_id: String) -> Result<Vec<OrderItem>, String> {
    println!("get_order_items called with order_id: {}", order_id);
    let order_id_parsed = order_id.parse::<i64>().map_err(|e| e.to_string())?;
    db::get_order_items_by_order_id(order_id_parsed).map_err(|e| {
        println!("get_order_items error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn search_sku_by_code(keyword: String) -> Result<Vec<SKU>, String> {
    println!("search_sku_by_code called with keyword: {}", keyword);
    let conn = get_db_path();
    let rusqlite_conn = rusqlite::Connection::open(conn).map_err(|e| e.to_string())?;
    
    if keyword.is_empty() {
        return Ok(vec![]);
    }
    
    let pattern = format!("%{}%", keyword);
    let mut stmt = rusqlite_conn.prepare(
        "SELECT s.id, s.sku_code, s.name, s.description, s.spec, s.unit, s.category_id, 
                s.box_spec, s.box_quantity, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
         FROM sku s 
         LEFT JOIN sku_category c ON s.category_id = c.category_id 
         WHERE s.sku_code LIKE ?1 AND s.is_deleted = 0
         LIMIT 10"
    ).map_err(|e| e.to_string())?;
    
    let skus = stmt.query_map([&pattern], |row| {
        Ok(SKU {
            id: Some(row.get(0)?),
            sku_code: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            spec: row.get(4)?,
            unit: row.get(5)?,
            category_id: row.get(6)?,
            category_name: row.get(12)?,
            box_spec: row.get(7)?,
            box_quantity: row.get(8)?,
            cost_price: row.get(9)?,
            sale_price: row.get(10)?,
            is_deleted: row.get::<_, i32>(11)? != 0,
        })
    }).map_err(|e| e.to_string())?;
    
    skus.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_order_item(data: OrderItem) -> Result<OrderItem, String> {
    println!("create_order_item called");
    db::create_order_item(data).map_err(|e| {
        println!("create_order_item error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn update_order_item(id: String, data: OrderItem) -> Result<OrderItem, String> {
    println!("update_order_item called with id: {}", id);
    let id_parsed = id.parse::<i64>().map_err(|e| e.to_string())?;
    db::update_order_item(id_parsed, data).map_err(|e| {
        println!("update_order_item error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn delete_order_item(id: String) -> Result<(), String> {
    println!("delete_order_item called with id: {}", id);
    let id_parsed = id.parse::<i64>().map_err(|e| e.to_string())?;
    db::delete_order_item(id_parsed).map_err(|e| {
        println!("delete_order_item error: {}", e);
        e.to_string()
    })
}
