use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct XivDbCategory {
  pub results: Vec<Value>,
  pub total: u64,
  pub paging: XivDbPaging
}

#[derive(Debug, Deserialize)]
pub struct XivDbPaging {
  pub page: u64,
  pub total: u64,
  pub pages: Vec<u64>,
  pub next: u64,
  pub prev: u64
}
