pub mod sku;
pub mod category;
pub mod customer;
pub mod order;
pub mod purchase;
pub mod financial;
pub mod common;
pub mod auth;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/auth")
                .route("/login", web::post().to(auth::login))
                .route("/logout", web::post().to(auth::logout))
                .route("/check", web::get().to(auth::check_auth))
        )
        .service(
            web::scope("/sku")
                .route("/list", web::get().to(sku::get_sku_list))
                .route("/list-paginated", web::get().to(sku::get_sku_list_paginated))
                .route("/get/{id}", web::get().to(sku::get_sku))
                .route("/create", web::post().to(sku::create_sku))
                .route("/update/{id}", web::put().to(sku::update_sku))
                .route("/delete/{id}", web::delete().to(sku::delete_sku))
                .route("/search", web::get().to(sku::search_sku))
                .route("/search-paginated", web::get().to(sku::search_sku_paginated))
                .route("/upload-image", web::post().to(sku::upload_sku_image))
        )
        .service(
            web::scope("/category")
                .route("/list", web::get().to(category::get_categories))
                .route("/create", web::post().to(category::create_category))
        )
        .service(
            web::scope("/customer")
                .route("/list", web::get().to(customer::get_customers))
                .route("/create", web::post().to(customer::create_customer))
        )
        .service(
            web::scope("/order")
                .route("/list", web::get().to(order::get_orders))
                .route("/get/{id}", web::get().to(order::get_order))
                .route("/create", web::post().to(order::create_order))
                .route("/update/{id}", web::put().to(order::update_order))
                .route("/delete/{id}", web::delete().to(order::delete_order))
        )
        .service(
            web::scope("/purchase")
                .route("/processing-orders", web::get().to(purchase::get_processing_orders))
                .route("/order-items/{order_id}", web::get().to(purchase::get_order_items))
                .route("/search-sku", web::get().to(purchase::search_sku_by_code))
                .route("/create-order-item", web::post().to(purchase::create_order_item))
                .route("/update-order-item/{id}", web::put().to(purchase::update_order_item))
                .route("/delete-order-item/{id}", web::delete().to(purchase::delete_order_item))
        )
        .service(
            web::scope("/financial")
                .route("/transactions", web::get().to(financial::get_transactions))
                .route("/create", web::post().to(financial::create_transaction))
        )
        .service(
            web::scope("/common")
                .route("/image/{sku_code}", web::get().to(common::get_image_url))
        );
}
