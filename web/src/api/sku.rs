use actix_web::{web, HttpResponse};
use crate::db::{self, models::SKU};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct SearchQuery {
    pub keyword: String,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

#[derive(Deserialize)]
pub struct SearchPaginationQuery {
    pub keyword: String,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

pub async fn get_sku_list() -> HttpResponse {
    match db::get_all_skus() {
        Ok(skus) => HttpResponse::Ok().json(skus),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_sku_list_paginated(query: web::Query<PaginationQuery>) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    
    match db::get_skus_paginated(page, page_size) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_sku(path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    match id.parse::<i64>() {
        Ok(id) => {
            match db::get_all_skus() {
                Ok(skus) => {
                    match skus.into_iter().find(|s| s.id == Some(id)) {
                        Some(sku) => HttpResponse::Ok().json(sku),
                        None => HttpResponse::NotFound().body("SKU not found"),
                    }
                },
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        },
        Err(_) => HttpResponse::BadRequest().body("Invalid ID"),
    }
}

pub async fn create_sku(sku: web::Json<SKU>) -> HttpResponse {
    match db::create_sku(sku.into_inner(), None) {
        Ok(sku) => HttpResponse::Ok().json(sku),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_sku(path: web::Path<String>, sku: web::Json<SKU>) -> HttpResponse {
    let id = path.into_inner();
    match id.parse::<i64>() {
        Ok(id) => match db::update_sku(id, sku.into_inner(), None) {
            Ok(sku) => HttpResponse::Ok().json(sku),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
        Err(_) => HttpResponse::BadRequest().body("Invalid ID"),
    }
}

pub async fn delete_sku(path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    match id.parse::<i64>() {
        Ok(id) => match db::delete_sku(id) {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
        Err(_) => HttpResponse::BadRequest().body("Invalid ID"),
    }
}

pub async fn search_sku(query: web::Query<SearchQuery>) -> HttpResponse {
    match db::search_skus(&query.keyword) {
        Ok(skus) => HttpResponse::Ok().json(skus),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn search_sku_paginated(query: web::Query<SearchPaginationQuery>) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    
    match db::search_skus_paginated(&query.keyword, page, page_size) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn upload_sku_image(
    _path: web::Path<String>,
    _data: web::Bytes,
    _oss: web::Data<crate::oss::OSSClientArc>,
) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Image upload not implemented yet"
    }))
}
