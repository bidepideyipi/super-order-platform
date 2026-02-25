use rusqlite::{Connection, Result};
use crate::db::{Order, get_db_path};

pub fn get_orders() -> Result<Vec<Order>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare(
        "SELECT o.*, c.customer_name 
         FROM `order` o 
         LEFT JOIN customer c ON o.customer_id = c.customer_id 
         ORDER BY o.created_at DESC"
    )?;
    let orders = stmt.query_map([], |row| {
        Ok(Order {
            id: Some(row.get(0)?),
            order_no: row.get(1)?,
            customer_id: row.get(2)?,
            order_date: row.get(4)?,
            status: row.get(5)?,
            total_cost_amount: row.get(6)?,
            total_sale_amount: row.get(7)?,
            remarks: row.get(8)?,
        })
    })?;
    
    orders.collect()
}
