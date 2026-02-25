use rusqlite::{Connection, Result};
use crate::db::{OrderItem, get_db_path};

pub fn get_order_items_by_order_id(order_id: i64) -> Result<Vec<OrderItem>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare(
        "SELECT oi.id, oi.order_id, oi.sku_id, oi.sku_code, oi.product_name, oi.quantity, 
                oi.cost_price, oi.sale_price, oi.total_cost_amount, oi.total_sale_amount,
                COALESCE(s.unit, '') as unit, COALESCE(s.box_spec, '') as box_spec
         FROM order_item oi
         LEFT JOIN sku s ON oi.sku_id = s.id
         WHERE oi.order_id = ?1 
         ORDER BY oi.id"
    )?;
    let items = stmt.query_map([order_id], |row| {
        Ok(OrderItem {
            id: Some(row.get(0)?),
            order_id: row.get(1)?,
            sku_id: row.get(2)?,
            sku_code: row.get(3)?,
            product_name: row.get(4)?,
            quantity: row.get(5)?,
            cost_price: row.get(6)?,
            sale_price: row.get(7)?,
            total_cost_amount: row.get(8)?,
            total_sale_amount: row.get(9)?,
            unit: row.get(10)?,
            box_spec: row.get(11)?,
        })
    })?;
    
    items.collect()
}

pub fn create_order_item(item: OrderItem) -> Result<OrderItem> {
    let conn = Connection::open(get_db_path())?;
    
    conn.execute(
        "INSERT INTO order_item (order_id, sku_id, sku_code, product_name, quantity, cost_price, sale_price, total_cost_amount, total_sale_amount)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        [
            &item.order_id as &dyn rusqlite::ToSql,
            &item.sku_id as &dyn rusqlite::ToSql,
            &item.sku_code as &dyn rusqlite::ToSql,
            &item.product_name as &dyn rusqlite::ToSql,
            &item.quantity as &dyn rusqlite::ToSql,
            &item.cost_price as &dyn rusqlite::ToSql,
            &item.sale_price as &dyn rusqlite::ToSql,
            &item.total_cost_amount as &dyn rusqlite::ToSql,
            &item.total_sale_amount as &dyn rusqlite::ToSql,
        ],
    )?;
    
    let id = conn.last_insert_rowid();
    let mut result = item;
    result.id = Some(id);
    Ok(result)
}

pub fn update_order_item(id: i64, item: OrderItem) -> Result<OrderItem> {
    let conn = Connection::open(get_db_path())?;
    
    conn.execute(
        "UPDATE order_item 
         SET sku_id = ?1, sku_code = ?2, product_name = ?3, quantity = ?4, 
             cost_price = ?5, sale_price = ?6, total_cost_amount = ?7, total_sale_amount = ?8 
         WHERE id = ?9",
        [
            &item.sku_id as &dyn rusqlite::ToSql,
            &item.sku_code as &dyn rusqlite::ToSql,
            &item.product_name as &dyn rusqlite::ToSql,
            &item.quantity as &dyn rusqlite::ToSql,
            &item.cost_price as &dyn rusqlite::ToSql,
            &item.sale_price as &dyn rusqlite::ToSql,
            &item.total_cost_amount as &dyn rusqlite::ToSql,
            &item.total_sale_amount as &dyn rusqlite::ToSql,
            &id as &dyn rusqlite::ToSql,
        ],
    )?;
    
    let mut result = item;
    result.id = Some(id);
    Ok(result)
}

pub fn delete_order_item(id: i64) -> Result<()> {
    let conn = Connection::open(get_db_path())?;
    
    conn.execute(
        "DELETE FROM order_item WHERE id = ?1",
        [id],
    )?;
    
    Ok(())
}
