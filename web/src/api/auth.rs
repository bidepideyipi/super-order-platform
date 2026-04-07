use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
}

pub async fn login(req: web::Json<LoginRequest>) -> HttpResponse {
    let valid_username = env::var("USER_NAME").unwrap_or_else(|_| "admin".to_string());
    let valid_password = env::var("USER_PASSWORD").unwrap_or_else(|_| "admin".to_string());

    if req.username == valid_username && req.password == valid_password {
        HttpResponse::Ok().json(LoginResponse {
            success: true,
            message: "登录成功".to_string(),
            token: Some(format!("{}:{}", req.username, chrono::Utc::now().timestamp())),
        })
    } else {
        HttpResponse::Unauthorized().json(LoginResponse {
            success: false,
            message: "用户名或密码错误".to_string(),
            token: None,
        })
    }
}

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "登出成功"
    }))
}

pub async fn check_auth() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "authenticated": true
    }))
}
