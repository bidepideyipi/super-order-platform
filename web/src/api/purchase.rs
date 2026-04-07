use actix_web::{web, HttpResponse};
use crate::db::models::{Order, OrderItem, SKU};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub keyword: String,
}

pub async fn get_processing_orders() -> HttpResponse {
    HttpResponse::Ok().json(Vec::<Order>::new())
}

pub async fn get_order_items(_path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().json(Vec::<OrderItem>::new())
}

pub async fn search_sku_by_code(_query: web::Query<SearchQuery>) -> HttpResponse {
    HttpResponse::Ok().json(Vec::<SKU>::new())
}

pub async fn create_order_item(_item: web::Json<OrderItem>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn update_order_item(_path: web::Path<String>, _item: web::Json<OrderItem>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn delete_order_item(_path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
