use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct SKU {
    pub id: Option<i64>,
    pub sku_code: Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: usize,
    pub page_size: usize,
    pub total_pages: usize,
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

pub fn get_skus_paginated(page: usize, page_size: usize) -> Result<PaginatedResult<SKU>> {
    let conn = Connection::open(get_db_path())?;
    
    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sku",
        [],
        |row| row.get(0)
    )?;
    
    let total_pages = if total == 0 {
        0
    } else {
        ((total as usize - 1) / page_size) + 1
    };
    
    let offset = (page - 1) * page_size;
    
    let mut stmt = conn.prepare(
        "SELECT s.*, c.category_name 
         FROM sku s 
         LEFT JOIN sku_category c ON s.category_id = c.category_id 
         ORDER BY s.id DESC 
         LIMIT ?1 OFFSET ?2"
    )?;
    
    let skus = stmt.query_map([page_size as i64, offset as i64], |row| {
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
    
    let data = skus.collect::<Result<Vec<_>, _>>()?;
    
    Ok(PaginatedResult {
        data,
        total,
        page,
        page_size,
        total_pages,
    })
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

pub fn search_skus(keyword: &str) -> Result<Vec<SKU>> {
    let conn = Connection::open(get_db_path())?;
    
    let pattern = format!("%{}%", keyword);
    
    let mut stmt = conn.prepare(
        "SELECT s.*, c.category_name 
         FROM sku s 
         LEFT JOIN sku_category c ON s.category_id = c.category_id 
         WHERE s.sku_code LIKE ?1 OR s.name LIKE ?1
         ORDER BY s.id DESC"
    )?;
    
    let skus = stmt.query_map([&pattern], |row| {
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

pub fn filter_skus_by_category(category_id: &str) -> Result<Vec<SKU>> {
    let conn = Connection::open(get_db_path())?;
    
    let mut stmt = conn.prepare(
        "SELECT s.*, c.category_name 
         FROM sku s 
         LEFT JOIN sku_category c ON s.category_id = c.category_id 
         WHERE s.category_id = ?1
         ORDER BY s.id DESC"
    )?;
    
    let skus = stmt.query_map([category_id], |row| {
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

pub fn search_skus_by_keyword_and_category(keyword: &str, category_id: Option<&str>) -> Result<Vec<SKU>> {
    if keyword.is_empty() && category_id.is_none() {
        return get_all_skus();
    }
    
    if keyword.is_empty() && category_id.is_some() {
        return filter_skus_by_category(category_id.unwrap());
    }
    
    let conn = Connection::open(get_db_path())?;
    let pattern = format!("%{}%", keyword);
    
    if let Some(cat_id) = category_id {
        let mut stmt = conn.prepare(
            "SELECT s.*, c.category_name 
             FROM sku s 
             LEFT JOIN sku_category c ON s.category_id = c.category_id 
             WHERE (s.sku_code LIKE ?1 OR s.name LIKE ?1) AND s.category_id = ?2
             ORDER BY s.id DESC"
        )?;
        let skus = stmt.query_map([&pattern, cat_id], |row| {
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
    } else {
        let mut stmt = conn.prepare(
            "SELECT s.*, c.category_name 
             FROM sku s 
             LEFT JOIN sku_category c ON s.category_id = c.category_id 
             WHERE s.sku_code LIKE ?1 OR s.name LIKE ?1
             ORDER BY s.id DESC"
        )?;
        let skus = stmt.query_map([&pattern], |row| {
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
}

pub fn generate_sku_code(category_id: &str) -> Result<String> {
    let conn = Connection::open(get_db_path())?;
    
    let max_seq: Option<i64> = conn.query_row(
        "SELECT MAX(CAST(SUBSTR(sku_code, 3) AS INTEGER)) 
         FROM sku 
         WHERE category_id = ?1",
        [category_id],
        |row| row.get(0)
    )?;
    
    let new_seq = max_seq.unwrap_or(0) + 1;
    let sku_code = format!("{}{:04}", category_id, new_seq);
    
    Ok(sku_code)
}

pub fn create_sku(mut sku: SKU) -> Result<SKU> {
    let conn = Connection::open(get_db_path())?;
    
    let sku_code = generate_sku_code(&sku.category_id.as_ref().unwrap_or(&"04".to_string()))?;
    sku.sku_code = Some(sku_code.clone());
    
    conn.execute(
        "INSERT INTO sku (sku_code, name, description, spec, unit, category_id, box_spec, cost_price, sale_price, image_path)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        [
            &sku.sku_code as &dyn rusqlite::ToSql,
            &sku.name as &dyn rusqlite::ToSql,
            &sku.description as &dyn rusqlite::ToSql,
            &sku.spec as &dyn rusqlite::ToSql,
            &sku.unit as &dyn rusqlite::ToSql,
            &sku.category_id as &dyn rusqlite::ToSql,
            &sku.box_spec as &dyn rusqlite::ToSql,
            &sku.cost_price as &dyn rusqlite::ToSql,
            &sku.sale_price as &dyn rusqlite::ToSql,
            &sku.image_path as &dyn rusqlite::ToSql,
        ],
    )?;
    
    let id = conn.last_insert_rowid();
    sku.id = Some(id);
    
    Ok(sku)
}
