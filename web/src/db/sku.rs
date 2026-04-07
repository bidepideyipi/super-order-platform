use rusqlite::{Connection, Result};
use crate::db::{SKU, PaginatedResult, get_db_path, get_images_dir};
use base64::Engine;
use std::fs;

pub fn get_all_skus() -> Result<Vec<SKU>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare(
        "SELECT s.id, s.sku_code, s.name, s.description, s.spec, s.unit, s.category_id, 
                s.box_spec, s.box_quantity, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
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
            category_name: row.get(12)?,
            box_spec: row.get(7)?,
            box_quantity: row.get(8)?,
            cost_price: row.get(9)?,
            sale_price: row.get(10)?,
            is_deleted: row.get::<_, i32>(11)? != 0,
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
                s.box_spec, s.box_quantity, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
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
            category_name: row.get(12)?,
            box_spec: row.get(7)?,
            box_quantity: row.get(8)?,
            cost_price: row.get(9)?,
            sale_price: row.get(10)?,
            is_deleted: row.get::<_, i32>(11)? != 0,
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

pub fn search_skus(keyword: &str) -> Result<Vec<SKU>> {
    let conn = Connection::open(get_db_path())?;
    
    let pattern = format!("%{}%", keyword);
    
    let mut stmt = conn.prepare(
        "SELECT s.id, s.sku_code, s.name, s.description, s.spec, s.unit, s.category_id, 
                s.box_spec, s.box_quantity, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
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
            category_name: row.get(12)?,
            box_spec: row.get(7)?,
            box_quantity: row.get(8)?,
            cost_price: row.get(9)?,
            sale_price: row.get(10)?,
            is_deleted: row.get::<_, i32>(11)? != 0,
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
                s.box_spec, s.box_quantity, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
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
            category_name: row.get(12)?,
            box_spec: row.get(7)?,
            box_quantity: row.get(8)?,
            cost_price: row.get(9)?,
            sale_price: row.get(10)?,
            is_deleted: row.get::<_, i32>(11)? != 0,
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
                s.box_spec, s.box_quantity, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
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
            category_name: row.get(12)?,
            box_spec: row.get(7)?,
            box_quantity: row.get(8)?,
            cost_price: row.get(9)?,
            sale_price: row.get(10)?,
            is_deleted: row.get::<_, i32>(11)? != 0,
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
                    s.box_spec, s.box_quantity, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
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
                category_name: row.get(12)?,
                box_spec: row.get(7)?,
                box_quantity: row.get(8)?,
                cost_price: row.get(9)?,
                sale_price: row.get(10)?,
                is_deleted: row.get::<_, i32>(11)? != 0,
            })
        })?;
        skus.collect()
    } else {
        let mut stmt = conn.prepare(
            "SELECT s.id, s.sku_code, s.name, s.description, s.spec, s.unit, s.category_id, 
                    s.box_spec, s.box_quantity, s.cost_price, s.sale_price, s.is_deleted, c.category_name 
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
                category_name: row.get(12)?,
                box_spec: row.get(7)?,
                box_quantity: row.get(8)?,
                cost_price: row.get(9)?,
                sale_price: row.get(10)?,
                is_deleted: row.get::<_, i32>(11)? != 0,
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
        "INSERT INTO sku (sku_code, name, description, spec, unit, category_id, box_spec, box_quantity, cost_price, sale_price, is_deleted)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 0)",
        [
            &sku.sku_code as &dyn rusqlite::ToSql,
            &sku.name as &dyn rusqlite::ToSql,
            &sku.description as &dyn rusqlite::ToSql,
            &sku.spec as &dyn rusqlite::ToSql,
            &sku.unit as &dyn rusqlite::ToSql,
            &sku.category_id as &dyn rusqlite::ToSql,
            &sku.box_spec as &dyn rusqlite::ToSql,
            &sku.box_quantity as &dyn rusqlite::ToSql,
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
         category_id = ?6, box_spec = ?7, box_quantity = ?8, cost_price = ?9, sale_price = ?10
         WHERE id = ?11",
        [
            &sku.sku_code as &dyn rusqlite::ToSql,
            &sku.name as &dyn rusqlite::ToSql,
            &sku.description as &dyn rusqlite::ToSql,
            &sku.spec as &dyn rusqlite::ToSql,
            &sku.unit as &dyn rusqlite::ToSql,
            &sku.category_id as &dyn rusqlite::ToSql,
            &sku.box_spec as &dyn rusqlite::ToSql,
            &sku.box_quantity as &dyn rusqlite::ToSql,
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
