use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use base64::Engine;

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
    pub is_deleted: bool,
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

pub fn get_images_dir() -> PathBuf {
    let base_dir = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    
    let project_root = base_dir
        .parent()
        .and_then(|p| p.parent())
        .unwrap_or_else(|| std::path::Path::new("."));
    
    let images_dir = project_root.join("desktop").join("public").join("images").join("sku");
    
    if !images_dir.exists() {
        fs::create_dir_all(&images_dir).unwrap_or_else(|e| {
            println!("Failed to create images directory: {}", e);
        });
    }
    
    images_dir
}

pub fn get_all_skus() -> Result<Vec<SKU>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare(
        "SELECT s.id, s.sku_code, s.name, s.description, s.spec, s.unit, s.category_id, 
                s.box_spec, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
         FROM sku s 
         LEFT JOIN sku_category c ON s.category_id = c.category_id 
         WHERE s.is_deleted = 0
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
            category_name: row.get(11)?,
            box_spec: row.get(7)?,
            cost_price: row.get(8)?,
            sale_price: row.get(9)?,
            is_deleted: row.get::<_, i32>(10)? != 0,
        })
    })?;
    
    skus.collect()
}

pub fn get_skus_paginated(page: usize, page_size: usize) -> Result<PaginatedResult<SKU>> {
    let conn = Connection::open(get_db_path())?;
    
    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sku WHERE is_deleted = 0",
        [],
        |row| row.get(0)
    )?;
    
    let total_pages = if total == 0 {
        0
    } else {
        ((total as usize - 1) / page_size) + 1
    };
    
    let offset = (page - 1) * page_size;
    
    let page_size_i32 = page_size as i32;
    let offset_i32 = offset as i32;
    
    let mut stmt = conn.prepare(
        "SELECT s.id, s.sku_code, s.name, s.description, s.spec, s.unit, s.category_id, 
                s.box_spec, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
         FROM sku s 
         LEFT JOIN sku_category c ON s.category_id = c.category_id 
         WHERE s.is_deleted = 0
         ORDER BY s.id DESC 
         LIMIT ?1 OFFSET ?2"
    )?;
    
    let skus = stmt.query_map([page_size_i32, offset_i32], |row| {
        Ok(SKU {
            id: Some(row.get(0)?),
            sku_code: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            spec: row.get(4)?,
            unit: row.get(5)?,
            category_id: row.get(6)?,
            category_name: row.get(11)?,
            box_spec: row.get(7)?,
            cost_price: row.get(8)?,
            sale_price: row.get(9)?,
            is_deleted: row.get::<_, i32>(10)? != 0,
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
        "SELECT s.id, s.sku_code, s.name, s.description, s.spec, s.unit, s.category_id, 
                s.box_spec, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
         FROM sku s 
         LEFT JOIN sku_category c ON s.category_id = c.category_id 
         WHERE (s.sku_code LIKE ?1 OR s.name LIKE ?1) AND s.is_deleted = 0
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
            category_name: row.get(11)?,
            box_spec: row.get(7)?,
            cost_price: row.get(8)?,
            sale_price: row.get(9)?,
            is_deleted: row.get::<_, i32>(10)? != 0,
        })
    })?;
    
    skus.collect()
}

pub fn search_skus_paginated(keyword: &str, page: usize, page_size: usize) -> Result<PaginatedResult<SKU>> {
    let conn = Connection::open(get_db_path())?;
    
    let pattern = format!("%{}%", keyword);
    
    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sku WHERE (sku_code LIKE ?1 OR name LIKE ?1) AND is_deleted = 0",
        [&pattern],
        |row| row.get(0)
    )?;
    
    let total_pages = if total == 0 {
        0
    } else {
        ((total as usize - 1) / page_size) + 1
    };
    
    let offset = (page - 1) * page_size;
    
    let page_size_i32 = page_size as i32;
    let offset_i32 = offset as i32;
    
    let mut stmt = conn.prepare(
        "SELECT s.id, s.sku_code, s.name, s.description, s.spec, s.unit, s.category_id, 
                s.box_spec, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
         FROM sku s 
         LEFT JOIN sku_category c ON s.category_id = c.category_id 
         WHERE (s.sku_code LIKE ?1 OR s.name LIKE ?1) AND s.is_deleted = 0
         ORDER BY s.id DESC 
         LIMIT ?2 OFFSET ?3"
    )?;
    
    let skus = stmt.query_map(
        rusqlite::params![&pattern, page_size_i32, offset_i32], 
        |row| {
        Ok(SKU {
            id: Some(row.get(0)?),
            sku_code: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            spec: row.get(4)?,
            unit: row.get(5)?,
            category_id: row.get(6)?,
            category_name: row.get(11)?,
            box_spec: row.get(7)?,
            cost_price: row.get(8)?,
            sale_price: row.get(9)?,
            is_deleted: row.get::<_, i32>(10)? != 0,
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

pub fn filter_skus_by_category(category_id: &str) -> Result<Vec<SKU>> {
    let conn = Connection::open(get_db_path())?;
    
    let mut stmt = conn.prepare(
        "SELECT s.id, s.sku_code, s.name, s.description, s.spec, s.unit, s.category_id, 
                s.box_spec, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
         FROM sku s 
         LEFT JOIN sku_category c ON s.category_id = c.category_id 
         WHERE s.category_id = ?1 AND s.is_deleted = 0
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
            category_name: row.get(11)?,
            box_spec: row.get(7)?,
            cost_price: row.get(8)?,
            sale_price: row.get(9)?,
            is_deleted: row.get::<_, i32>(10)? != 0,
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
            "SELECT s.id, s.sku_code, s.name, s.description, s.spec, s.unit, s.category_id, 
                    s.box_spec, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
             FROM sku s 
             LEFT JOIN sku_category c ON s.category_id = c.category_id 
             WHERE (s.sku_code LIKE ?1 OR s.name LIKE ?1) AND s.category_id = ?2 AND s.is_deleted = 0
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
                category_name: row.get(11)?,
                box_spec: row.get(7)?,
                cost_price: row.get(8)?,
                sale_price: row.get(9)?,
                is_deleted: row.get::<_, i32>(10)? != 0,
            })
        })?;
        skus.collect()
    } else {
        let mut stmt = conn.prepare(
            "SELECT s.id, s.sku_code, s.name, s.description, s.spec, s.unit, s.category_id, 
                    s.box_spec, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
             FROM sku s 
             LEFT JOIN sku_category c ON s.category_id = c.category_id 
             WHERE (s.sku_code LIKE ?1 OR s.name LIKE ?1) AND s.is_deleted = 0
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
                category_name: row.get(11)?,
                box_spec: row.get(7)?,
                cost_price: row.get(8)?,
                sale_price: row.get(9)?,
                is_deleted: row.get::<_, i32>(10)? != 0,
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

pub fn create_sku(mut sku: SKU, image_base64: Option<String>) -> Result<SKU> {
    let conn = Connection::open(get_db_path())?;
    
    let sku_code = generate_sku_code(&sku.category_id.as_ref().unwrap_or(&"04".to_string()))?;
    sku.sku_code = Some(sku_code.clone());
    
    conn.execute(
        "INSERT INTO sku (sku_code, name, description, spec, unit, category_id, box_spec, cost_price, sale_price, is_deleted)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 0)",
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
        ],
    )?;
    
    let id = conn.last_insert_rowid();
    sku.id = Some(id);
    
    if let Some(img_data) = image_base64 {
        let image_bytes: Vec<u8> = base64::engine::general_purpose::STANDARD.decode(&img_data)
            .map_err(|e: base64::DecodeError| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
        
        let images_dir = get_images_dir();
        let image_path = images_dir.join(format!("{}.jpeg", sku_code));
        
        fs::write(&image_path, image_bytes)
            .map_err(|e: std::io::Error| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
    }
    
    Ok(sku)
}

pub fn update_sku(id: i64, mut sku: SKU, image_base64: Option<String>) -> Result<SKU> {
    let conn = Connection::open(get_db_path())?;
    
    if let Some(img_data) = image_base64 {
        if let Some(sku_code) = &sku.sku_code {
            let images_dir = get_images_dir();
            let image_path = images_dir.join(format!("{}.jpeg", sku_code));
            
            let image_bytes: Vec<u8> = base64::engine::general_purpose::STANDARD.decode(&img_data)
                .map_err(|e: base64::DecodeError| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
            
            fs::write(&image_path, image_bytes)
                .map_err(|e: std::io::Error| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
        }
    }
    
    conn.execute(
        "UPDATE sku SET sku_code = ?1, name = ?2, description = ?3, spec = ?4, unit = ?5, 
         category_id = ?6, box_spec = ?7, cost_price = ?8, sale_price = ?9
         WHERE id = ?10",
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
            &id as &dyn rusqlite::ToSql,
        ],
    )?;
    
    sku.id = Some(id);
    Ok(sku)
}

pub fn delete_sku(id: i64) -> Result<()> {
    let conn = Connection::open(get_db_path())?;
    
    conn.execute(
        "UPDATE sku SET is_deleted = 1 WHERE id = ?1",
        [id],
    )?;
    
    Ok(())
}
