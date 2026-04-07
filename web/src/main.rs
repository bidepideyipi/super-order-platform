use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use super_order_web::{config, oss};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let config = config::Config::from_env();
    
    let oss_client = Arc::new(oss::OSSClient::new(
        config.oss_region.clone(),
        config.oss_bucket_name.clone(),
        config.oss_access_key_id.clone(),
        config.oss_access_key_secret.clone(),
    ));

    log::info!("Starting server at {}:{}", config.server_host, config.server_port);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(oss_client.clone()))
            .service(
                web::scope("/api")
                    .configure(super_order_web::api::configure_routes)
            )
    })
    .bind((config.server_host.as_str(), config.server_port))?
    .run()
    .await
}
