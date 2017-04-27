use serde_json::Value;

use kind::patch::XivDbPatch;

#[derive(Debug, Deserialize)]
pub struct XivDbTitle {
  pub achievements: Vec<Value>,
  pub icon: String,
  pub icon_hq: String,
  pub id: u64,
  pub is_prefix: u8,
  pub lodestone_id: Option<String>,
  pub lodestone_type: Option<String>,
  pub name: String,
  pub name_cns: String,
  pub name_de: String,
  pub name_en: String,
  pub name_female: String,
  pub name_female_cns: String,
  pub name_female_de: String,
  pub name_female_en: String,
  pub name_female_fr: String,
  pub name_female_ja: String,
  pub name_fr: String,
  pub name_ja: String,
  pub patch: XivDbPatch,
  pub url: String,
  pub url_api: String,
  pub url_type: String,
  pub url_xivdb: String,
  pub url_xivdb_de: String,
  pub url_xivdb_fr: String,
  pub url_xivdb_ja: String,
  pub _cid: u64,
  pub _type: String,
}
