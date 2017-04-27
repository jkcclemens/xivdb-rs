use serde_json::Value;
use kind::patch::XivDbPatch;

#[derive(Debug, Deserialize)]
pub struct XivDbEnemy {
  pub connect_area: u64,
  pub connect_instance: u64,
  pub connect_items: u64,
  pub connect_nonpop: u64,
  pub icon: String,
  pub icon_hq: String,
  pub id: u64,
  pub instances: Option<Value>,
  pub items: Option<Value>,
  pub lodestone_id: String,
  pub lodestone_type: String,
  pub map: u64,
  pub map_data: String,
  pub map_primary: Option<Value>,
  pub name: String,
  pub name_cns: String,
  pub name_de: String,
  pub name_en: String,
  pub name_fr: String,
  pub name_ja: String,
  pub name_plural_cns: String,
  pub name_plural_de: String,
  pub name_plural_en: String,
  pub name_plural_fr: String,
  pub name_plural_ja: String,
  pub patch: XivDbPatch,
  pub placename: u64,
  pub position: String,
  pub url: String,
  pub url_api: String,
  pub url_lodestone: String,
  pub url_type: String,
  pub url_xivdb: String,
  pub url_xivdb_de: String,
  pub url_xivdb_fr: String,
  pub url_xivdb_ja: String,
  pub xyz: u64,
  pub _cid: u64,
  pub _type: String
}