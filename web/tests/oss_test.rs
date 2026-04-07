use std::sync::Arc;

mod common;

#[tokio::test]
async fn test_oss_client_new() {
    let client = common::create_oss_client();
    assert_eq!(client.bucket, common::get_test_bucket());
}

#[tokio::test]
async fn test_upload() {
    let client = common::create_oss_client();
    
    let test_data = vec![1, 2, 3, 4, 5];
    let result = client.upload("test-key.txt", test_data).await;
    
    if let Err(ref e) = result {
        eprintln!("Upload error: {}", e);
    }
    
    assert!(result.is_ok(), "Upload should succeed");
    assert_eq!(result.unwrap(), "test-key.txt");
}

#[tokio::test]
async fn test_get_url() {
    let client = common::create_oss_client();
    
    let result = client.get_url("test-key.txt", 3600).await;
    
    assert!(result.is_ok());
    let url = result.unwrap();
    
    assert!(url.contains(&common::get_test_bucket()));
    assert!(url.contains("OSSAccessKeyId="));
    assert!(url.contains("Expires="));
    assert!(url.contains("Signature="));
}

#[tokio::test]
async fn test_upload_with_different_keys() {
    let client = common::create_oss_client();
    
    let test_cases = vec![
        "images/sku/010001.jpeg",
        "documents/report.pdf",
        "data/export.xlsx",
    ];
    
    for key in test_cases {
        let result = client.upload(key, vec![]).await;
        if let Err(ref e) = result {
            eprintln!("Upload error for key '{}': {}", key, e);
        }
        assert!(result.is_ok(), "Upload should succeed for key: {}", key);
        assert_eq!(result.unwrap(), key);
    }
}

#[tokio::test]
async fn test_get_url_with_different_expires() {
    let client = common::create_oss_client();
    
    let expires_values = vec![3600, 7200, 86400];
    
    for expires in expires_values {
        let result = client.get_url("test-key.txt", expires).await;
        assert!(result.is_ok());
        let url = result.unwrap();
        assert!(url.contains(&common::get_test_bucket()));
        assert!(url.contains("Signature="));
    }
}

#[tokio::test]
async fn test_oss_client_arc() {
    let client = Arc::new(common::create_oss_client());
    let client_clone = Arc::clone(&client);
    
    let result1 = client.upload("test1.txt", vec![]).await;
    if let Err(ref e) = result1 {
        eprintln!("Upload error for test1.txt: {}", e);
    }
    
    let result2 = client_clone.upload("test2.txt", vec![]).await;
    if let Err(ref e) = result2 {
        eprintln!("Upload error for test2.txt: {}", e);
    }
    
    assert!(result1.is_ok(), "Upload test1.txt should succeed");
    assert!(result2.is_ok(), "Upload test2.txt should succeed");
}

#[tokio::test]
async fn test_env_configuration() {
    let bucket = common::get_test_bucket();
    
    assert_eq!(bucket, "super-order-images", "应该使用 .env 文件中的配置");
    
    let client = common::create_oss_client();
    assert_eq!(client.bucket, "super-order-images");
}
