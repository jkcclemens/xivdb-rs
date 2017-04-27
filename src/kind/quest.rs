use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct XivDbQuest {
  pub class_1: String,
  pub class_level_1: u64,
  pub class_level_2: u64,
  pub classjob_category_1: Option<String>,
  pub classjob_category_2: Option<String>,
  pub genre_icon: String,
  pub genre_name: String,
  pub icon: String,
  pub id: u64,
  pub items: Option<Value>,
  pub items_total: u64,
  pub journal_genre: u64,
  pub name: String,
  pub url: String,
  pub url_api: String,
  pub url_type: String,
  pub url_xivdb: String,
}
