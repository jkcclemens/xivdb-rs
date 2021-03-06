use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct XivDbGathering {
  pub color: String,
  pub gathering_item_number: u64,
  pub gathering_notebook_list: u64,
  pub gathering_type: u64,
  pub icon: String,
  pub icon_lodestone: String,
  pub id: u64,
  pub is_hidden: u64,
  pub item: Value,
  pub item_level_equip: u64,
  pub item_level_item: u64,
  pub level: u64,
  pub level_diff: u64,
  pub level_view: u64,
  pub lodestone_id: String,
  pub lodestone_type: String,
  pub name: String,
  pub name_cns: String,
  pub name_de: String,
  pub name_en: String,
  pub name_fr: String,
  pub name_ja: String,
  pub nodes: Value,
  pub patch: Value,
  pub placename: String,
  pub rarity: u64,
  pub stars: u64,
  pub stars_html: String,
  pub type_name: String,
  pub url: String,
  pub url_api: String,
  pub url_lodestone: String,
  pub url_type: String,
  pub url_xivdb: String,
  pub url_xivdb_de: String,
  pub url_xivdb_fr: String,
  pub url_xivdb_ja: String,
  pub _cid: i64,
  pub _type: String,
}
