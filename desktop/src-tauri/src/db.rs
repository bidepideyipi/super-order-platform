use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct SKU {
    pub id: Option<i64>,
    pub sku_code: String,
    pub name: String,
    pub description: Option<String>,
    pub spec: Option<String>,
    pub unit: String,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub box_spec: Option<String>,
    pub cost_price: f64,
    pub sale_price: f64,
    pub image_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub category_id: String,
    pub category_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub customer_id: String,
    pub customer_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: Option<i64>,
    pub order_no: String,
    pub customer_id: String,
    pub customer_name: String,
    pub order_date: String,
    pub status: String,
    pub total_amount: f64,
    pub remarks: Option<String>,
}

pub fn get_db_path() -> PathBuf {
    let base_dir = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    
    let project_root = base_dir
        .parent()
        .and_then(|p| p.parent())
        .unwrap_or_else(|| std::path::Path::new("."));
    
    println!("Database path calculation: base_dir = {:?}, project_root = {:?}", base_dir, project_root);
    
    project_root.join("data").join("super_order.db")
}

pub fn get_all_skus() -> Result<Vec<SKU>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare(
        "SELECT s.*, c.category_name 
         FROM sku s 
         LEFT JOIN sku_category c ON s.category_id = c.category_id 
         ORDER BY s.id DESC"
    )?;
    let skus = stmt.query_map([], |row| {
        Ok(SKU {
            id: Some(row.get(0)?),
            sku_code: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            spec: row.get(4)?,
            unit: row.get(5)?,
            category_id: row.get(6)?,
            category_name: row.get(13)?,
            box_spec: row.get(7)?,
            cost_price: row.get(8)?,
            sale_price: row.get(9)?,
            image_path: row.get(10)?,
        })
    })?;
    
    skus.collect()
}

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
            customer_name: row.get(10)?,
            order_date: row.get(4)?,
            status: row.get(5)?,
            total_amount: row.get(6)?,
            remarks: row.get(7)?,
        })
    })?;
    
    orders.collect()
}
