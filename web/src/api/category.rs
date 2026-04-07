use actix_web::{web, HttpResponse};
use crate::db::models::Category;

pub async fn get_categories() -> HttpResponse {
    HttpResponse::Ok().json(Vec::<Category>::new())
}

pub async fn create_category(_category: web::Json<Category>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
