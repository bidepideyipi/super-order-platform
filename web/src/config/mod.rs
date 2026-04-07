use std::env;

pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub oss_region: String,
    pub oss_bucket_name: String,
    pub oss_access_key_id: String,
    pub oss_access_key_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        
        Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "data/super_order.db".to_string()),
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            oss_region: env::var("OSS_REGION")
                .unwrap_or_else(|_| "cn-shenzhen".to_string()),
            oss_bucket_name: env::var("OSS_BUCKET_NAME")
                .unwrap_or_else(|_| "super-order-images".to_string()),
            oss_access_key_id: env::var("ALIBABA_CLOUD_ACCESS_KEY_ID")
                .expect("ALIBABA_CLOUD_ACCESS_KEY_ID must be set"),
            oss_access_key_secret: env::var("ALIBABA_CLOUD_ACCESS_KEY_SECRET")
                .expect("ALIBABA_CLOUD_ACCESS_KEY_SECRET must be set"),
        }
    }
}
