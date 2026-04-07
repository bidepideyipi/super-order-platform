use rusqlite::{Connection, Result};
use crate::db::{Category, get_db_path};

pub fn get_categories() -> Result<Vec<Category>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare("SELECT * FROM sku_category ORDER BY category_id")?;
    let categories = stmt.query_map([], |row| {
        Ok(Category {
            category_id: row.get(0)?,
            category_name: row.get(1)?,
        })
    })?;
    
    categories.collect()
}
