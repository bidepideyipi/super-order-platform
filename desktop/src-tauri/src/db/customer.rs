use rusqlite::{Connection, Result};
use crate::db::{Customer, get_db_path};

pub fn get_customers() -> Result<Vec<Customer>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare("SELECT * FROM customer ORDER BY customer_id")?;
    let customers = stmt.query_map([], |row| {
        Ok(Customer {
            customer_id: row.get(0)?,
            customer_name: row.get(1)?,
        })
    })?;
    
    customers.collect()
}
