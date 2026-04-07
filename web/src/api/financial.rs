use actix_web::{web, HttpResponse};
use crate::db::models::FinancialTransaction;

pub async fn get_transactions() -> HttpResponse {
    HttpResponse::Ok().json(Vec::<FinancialTransaction>::new())
}

pub async fn create_transaction(_transaction: web::Json<FinancialTransaction>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
