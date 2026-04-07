use actix_web::{web, HttpResponse};
use crate::db::models::Customer;

pub async fn get_customers() -> HttpResponse {
    HttpResponse::Ok().json(Vec::<Customer>::new())
}

pub async fn create_customer(_customer: web::Json<Customer>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
