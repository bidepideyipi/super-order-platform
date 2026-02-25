use rusqlite::{Connection, Result};
use crate::db::{OrderItem, get_db_path};

pub fn get_order_items_by_order_id(order_id: i64) -> Result<Vec<OrderItem>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare(
        "SELECT id, order_id, sku_id, sku_code, product_name, quantity, unit_price, subtotal 
         FROM order_item 
         WHERE order_id = ?1 
         ORDER BY id"
    )?;
    let items = stmt.query_map([order_id], |row| {
        Ok(OrderItem {
            id: Some(row.get(0)?),
            order_id: row.get(1)?,
            sku_id: row.get(2)?,
            sku_code: row.get(3)?,
            product_name: row.get(4)?,
            quantity: row.get(5)?,
            unit_price: row.get(6)?,
            subtotal: row.get(7)?,
        })
    })?;
    
    items.collect()
}
