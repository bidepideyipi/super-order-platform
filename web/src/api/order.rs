use actix_web::{web, HttpResponse};
use crate::db::models::Order;

pub async fn get_orders() -> HttpResponse {
    HttpResponse::Ok().json(Vec::<Order>::new())
}

pub async fn get_order(_path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn create_order(_order: web::Json<Order>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn update_order(_path: web::Path<String>, _order: web::Json<Order>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn delete_order(_path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
