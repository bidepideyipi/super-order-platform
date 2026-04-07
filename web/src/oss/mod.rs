use std::sync::Arc;
use chrono::Utc;

mod signature;

pub struct OSSClient {
    pub bucket: String,
    region: String,
    access_key_id: String,
    access_key_secret: String,
}

impl OSSClient {
    pub fn new(
        region: String,
        bucket_name: String,
        access_key_id: String,
        access_key_secret: String,
    ) -> Self {
        OSSClient {
            bucket: bucket_name,
            region,
            access_key_id,
            access_key_secret,
        }
    }

    pub async fn upload(&self, key: &str, data: Vec<u8>) -> Result<String, String> {
        let url = format!(
            "https://{}.oss-{}.aliyuncs.com/{}",
            self.bucket, self.region, key
        );

        let date = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let content_type = "application/octet-stream";
        let content_length = data.len();
        
        let string_to_sign = format!(
            "PUT\n\n{}\n{}\n/{}/{}",
            content_type, date, self.bucket, key
        );

        let signature = signature::generate_signature(&self.access_key_secret, &string_to_sign);
        let authorization = format!("OSS {}:{}", self.access_key_id, signature);

        let client = reqwest::Client::new();
        let response = client
            .put(&url)
            .header("Date", date)
            .header("Authorization", authorization)
            .header("Content-Type", content_type)
            .header("Content-Length", content_length)
            .body(data)
            .send()
            .await
            .map_err(|e| format!("Upload failed: {}", e))?;

        if response.status().is_success() {
            log::info!("Upload completed - key: {}", key);
            Ok(key.to_string())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| "Unable to read response body".to_string());
            Err(format!("Upload failed with status: {}, body: {}", status, body))
        }
    }

    pub async fn get_url(&self, key: &str, expires: i64) -> Result<String, String> {
        let expires_time = Utc::now().timestamp() + expires;
        
        let string_to_sign = format!(
            "GET\n\n\n{}\n/{}/{}",
            expires_time, self.bucket, key
        );

        let signature = signature::generate_signature(&self.access_key_secret, &string_to_sign);
        let encoded_signature = signature::url_encode(&signature);

        let url = format!(
            "https://{}.oss-{}.aliyuncs.com/{}?OSSAccessKeyId={}&Expires={}&Signature={}",
            self.bucket, self.region, key, self.access_key_id, expires_time, encoded_signature
        );

        log::info!("Generated signed URL - key: {}, expires: {}s", key, expires);
        Ok(url)
    }
}

pub type OSSClientArc = Arc<OSSClient>;
