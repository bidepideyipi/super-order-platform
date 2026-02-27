use rusqlite::{Connection, Result};
use crate::db::{OrderItem, get_db_path};

pub fn get_order_items_by_order_id(order_id: i64) -> Result<Vec<OrderItem>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare(
        "SELECT oi.id, oi.order_id, oi.sku_id, oi.sku_code, oi.product_name, oi.quantity, 
                oi.cost_price, oi.sale_price, oi.total_cost_amount, oi.total_sale_amount,
                COALESCE(s.unit, '') as unit, COALESCE(s.box_spec, '') as box_spec,
                COALESCE(s.box_quantity, 1) as box_quantity, COALESCE(s.spec, '') as spec
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
            box_quantity: row.get(12)?,
            spec: row.get(13)?,
        })
    })?;
    
    items.collect()
}

fn update_order_totals(conn: &Connection, order_id: i64) -> Result<()> {
    conn.execute(
        "UPDATE `order` 
         SET total_cost_amount = (
             SELECT COALESCE(SUM(total_cost_amount), 0) 
             FROM order_item 
             WHERE order_id = ?1
         ),
         total_sale_amount = (
             SELECT COALESCE(SUM(total_sale_amount), 0) 
             FROM order_item 
             WHERE order_id = ?1
         )
         WHERE id = ?1",
        [order_id],
    )?;
    Ok(())
}

pub fn create_order_item(item: OrderItem) -> Result<OrderItem> {
    let mut conn = Connection::open(get_db_path())?;
    let tx = conn.transaction()?;
    
    tx.execute(
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
    
    let id = tx.last_insert_rowid();
    
    // 更新订单总金额
    update_order_totals(&tx, item.order_id)?;
    
    tx.commit()?;
    
    let mut result = item;
    result.id = Some(id);
    Ok(result)
}

pub fn update_order_item(id: i64, item: OrderItem) -> Result<OrderItem> {
    let mut conn = Connection::open(get_db_path())?;
    let tx = conn.transaction()?;
    
    // 先获取原来的order_id
    let original_order_id: i64 = tx.query_row(
        "SELECT order_id FROM order_item WHERE id = ?1",
        [id],
        |row| row.get(0)
    )?;
    
    tx.execute(
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
    
    // 更新订单总金额（使用原来的order_id，因为item.order_id可能为空）
    update_order_totals(&tx, original_order_id)?;
    
    tx.commit()?;
    
    let mut result = item;
    result.id = Some(id);
    Ok(result)
}

pub fn delete_order_item(id: i64) -> Result<()> {
    let mut conn = Connection::open(get_db_path())?;
    let tx = conn.transaction()?;
    
    // 先获取order_id
    let order_id: i64 = tx.query_row(
        "SELECT order_id FROM order_item WHERE id = ?1",
        [id],
        |row| row.get(0)
    )?;
    
    tx.execute(
        "DELETE FROM order_item WHERE id = ?1",
        [id],
    )?;
    
    // 更新订单总金额
    update_order_totals(&tx, order_id)?;
    
    tx.commit()?;
    
    Ok(())
}
