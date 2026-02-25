use rusqlite::{Connection, Result};
use crate::db::{Order, get_db_path};
use chrono::Utc;

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
            order_date: row.get(3)?,
            status: row.get(4)?,
            total_cost_amount: row.get(5)?,
            total_sale_amount: row.get(6)?,
            remarks: row.get(7)?,
        })
    })?;
    
    orders.collect()
}

pub fn get_order(id: i64) -> Result<Option<Order>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare(
        "SELECT o.*, c.customer_name 
         FROM `order` o 
         LEFT JOIN customer c ON o.customer_id = c.customer_id 
         WHERE o.id = ?1"
    )?;
    
    let order = stmt.query_row([id], |row| {
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
    });
    
    match order {
        Ok(order) => Ok(Some(order)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn create_order(mut order: Order) -> Result<Order> {
    let conn = Connection::open(get_db_path())?;
    
    let order_no = generate_order_no(&conn, &order.customer_id)?;
    order.order_no = Some(order_no.clone());
    
    conn.execute(
        "INSERT INTO `order` (order_no, customer_id, order_date, status, total_cost_amount, total_sale_amount, remarks)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        [
            &order_no as &dyn rusqlite::ToSql,
            &order.customer_id as &dyn rusqlite::ToSql,
            &order.order_date as &dyn rusqlite::ToSql,
            &order.status as &dyn rusqlite::ToSql,
            &order.total_cost_amount as &dyn rusqlite::ToSql,
            &order.total_sale_amount as &dyn rusqlite::ToSql,
            &order.remarks as &dyn rusqlite::ToSql,
        ],
    )?;
    
    let id = conn.last_insert_rowid();
    order.id = Some(id);
    Ok(order)
}

pub fn update_order(id: i64, mut order: Order) -> Result<Order> {
    let conn = Connection::open(get_db_path())?;
    
    conn.execute(
        "UPDATE `order` 
         SET customer_id = ?1, order_date = ?2, status = ?3, 
             total_cost_amount = ?4, total_sale_amount = ?5, remarks = ?6 
         WHERE id = ?7",
        [
            &order.customer_id as &dyn rusqlite::ToSql,
            &order.order_date as &dyn rusqlite::ToSql,
            &order.status as &dyn rusqlite::ToSql,
            &order.total_cost_amount as &dyn rusqlite::ToSql,
            &order.total_sale_amount as &dyn rusqlite::ToSql,
            &order.remarks as &dyn rusqlite::ToSql,
            &id as &dyn rusqlite::ToSql,
        ],
    )?;
    
    order.id = Some(id);
    Ok(order)
}

pub fn delete_order(id: i64) -> Result<()> {
    let conn = Connection::open(get_db_path())?;
    
    conn.execute(
        "DELETE FROM `order` WHERE id = ?1",
        [id],
    )?;
    
    Ok(())
}

fn generate_order_no(conn: &Connection, customer_id: &str) -> Result<String> {
    let now = Utc::now();
    let date_str = now.format("%Y%m%d").to_string();
    
    let today_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM `order` WHERE customer_id = ?1 AND DATE(order_date) = DATE('now', 'localtime')",
        [customer_id],
        |row| row.get(0)
    )?;
    
    let customer_prefix = if customer_id.len() > 3 {
        &customer_id[..3]
    } else {
        customer_id
    };
    
    let _sequence = format!("{:03}", today_count + 1);
    let order_no = format!("{}{}", customer_prefix, date_str);
    
    Ok(order_no)
}
