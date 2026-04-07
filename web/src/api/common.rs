use actix_web::{web, HttpResponse};
use crate::oss::OSSClientArc;

pub async fn get_image_url(
    path: web::Path<String>,
    oss: web::Data<OSSClientArc>,
) -> HttpResponse {
    let sku_code = path.into_inner();
    let key = format!("sku/{}.jpeg", sku_code);
    
    match oss.get_url(&key, 3600).await {
        Ok(url) => HttpResponse::Ok().json(serde_json::json!({
            "url": url
        })),
        Err(e) => {
            log::error!("Failed to generate image URL: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e
            }))
        }
    }
}
