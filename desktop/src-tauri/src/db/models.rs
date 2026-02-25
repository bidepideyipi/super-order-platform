use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SKU {
    pub id: Option<i64>,
    pub sku_code: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub spec: Option<String>,
    pub unit: String,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub box_spec: Option<String>,
    #[serde(default)]
    pub cost_price: f64,
    #[serde(default)]
    pub sale_price: f64,
    #[serde(default)]
    pub is_deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub category_id: String,
    pub category_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub customer_id: String,
    pub customer_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: Option<i64>,
    pub order_no: String,
    pub customer_id: String,
    pub customer_name: String,
    pub order_date: String,
    pub status: String,
    pub total_amount: f64,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: usize,
    pub page_size: usize,
    pub total_pages: usize,
}
