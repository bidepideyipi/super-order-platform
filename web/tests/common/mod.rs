use super_order_web::oss::OSSClient;
use std::env;

pub fn create_oss_client() -> OSSClient {
    dotenv::dotenv().ok();
    
    let region = env::var("OSS_REGION")
        .unwrap_or_else(|_| "cn-shenzhen".to_string());
    
    let bucket_name = env::var("OSS_BUCKET_NAME")
        .unwrap_or_else(|_| "test-bucket".to_string());
    
    let access_key_id = env::var("ALIBABA_CLOUD_ACCESS_KEY_ID")
        .unwrap_or_else(|_| "test-access-key-id".to_string());
    
    let access_key_secret = env::var("ALIBABA_CLOUD_ACCESS_KEY_SECRET")
        .unwrap_or_else(|_| "test-access-key-secret".to_string());
    
    OSSClient::new(region, bucket_name, access_key_id, access_key_secret)
}

pub fn get_test_bucket() -> String {
    dotenv::dotenv().ok();
    
    env::var("OSS_BUCKET_NAME")
        .unwrap_or_else(|_| "test-bucket".to_string())
}

#[allow(dead_code)]
pub fn get_test_region() -> String {
    dotenv::dotenv().ok();
    
    env::var("OSS_REGION")
        .unwrap_or_else(|_| "cn-shenzhen".to_string())
}

#[allow(dead_code)]
pub fn setup() {
    dotenv::dotenv().ok();
    
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::try_init();
}
