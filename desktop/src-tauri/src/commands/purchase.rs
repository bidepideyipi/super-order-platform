use crate::db::{self, Order, OrderItem, get_db_path};

#[tauri::command]
pub fn get_processing_orders() -> Result<Vec<Order>, String> {
    println!("get_processing_orders called");
    let conn = get_db_path();
    let rusqlite_conn = rusqlite::Connection::open(conn).map_err(|e| e.to_string())?;
    
    let mut stmt = rusqlite_conn.prepare(
        "SELECT o.*, c.customer_name 
         FROM `order` o 
         LEFT JOIN customer c ON o.customer_id = c.customer_id 
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
            total_cost_amount: row.get(5)?,
            total_sale_amount: row.get(6)?,
            remarks: row.get(7)?,
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
