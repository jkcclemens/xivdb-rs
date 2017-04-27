#[derive(Debug, Deserialize)]
pub struct XivDbInstance {
  pub content_icon: Option<String>,
  pub content_name: String,
  pub icon: String,
  pub icon_mini: String,
  pub id: u64,
  pub item_level: u64,
  pub item_level_sync: u64,
  pub level: u64,
  pub level_sync: u64,
  pub name: String,
  pub url: String,
  pub url_api: String,
  pub url_type: String,
  pub url_xivdb: String,
}
