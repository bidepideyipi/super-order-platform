use rusqlite::{Connection, Result};
use crate::db::{Order, get_db_path};

pub fn get_orders() -> Result<Vec<Order>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare(
        "SELECT o.id, o.order_no, o.customer_id, o.order_date, o.status, o.is_settled, 
                o.total_cost_amount, o.total_sale_amount, o.remarks
         FROM `order` o 
         ORDER BY o.created_at DESC"
    )?;
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
    })?;
    
    orders.collect()
}

pub fn get_order(id: i64) -> Result<Option<Order>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare(
        "SELECT o.id, o.order_no, o.customer_id, o.order_date, o.status, o.is_settled, 
                o.total_cost_amount, o.total_sale_amount, o.remarks
         FROM `order` o 
         WHERE o.id = ?1"
    )?;
    
    let order = stmt.query_row([id], |row| {
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
    });
    
    match order {
        Ok(order) => Ok(Some(order)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn create_order(mut order: Order) -> Result<Order> {
    let conn = Connection::open(get_db_path())?;
    
    let order_no = generate_order_no(&order.customer_id, &order.order_date)?;
    order.order_no = Some(order_no.clone());
    
    conn.execute(
        "INSERT INTO `order` (order_no, customer_id, order_date, status, is_settled, total_cost_amount, total_sale_amount, remarks)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        [
            &order_no as &dyn rusqlite::ToSql,
            &order.customer_id as &dyn rusqlite::ToSql,
            &order.order_date as &dyn rusqlite::ToSql,
            &order.status as &dyn rusqlite::ToSql,
            &(order.is_settled as i32) as &dyn rusqlite::ToSql,
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
         SET customer_id = ?1, order_date = ?2, status = ?3, is_settled = ?4, 
             total_cost_amount = ?5, total_sale_amount = ?6, remarks = ?7 
         WHERE id = ?8",
        [
            &order.customer_id as &dyn rusqlite::ToSql,
            &order.order_date as &dyn rusqlite::ToSql,
            &order.status as &dyn rusqlite::ToSql,
            &(order.is_settled as i32) as &dyn rusqlite::ToSql,
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

fn generate_order_no(customer_id: &str, order_date: &str) -> Result<String> {
    let formatted_date = order_date.replace("-", "");
    let order_no = format!("{}{}", customer_id, formatted_date);
    Ok(order_no)
}
