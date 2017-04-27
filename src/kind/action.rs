use serde_json::Value;

use kind::patch::XivDbPatch;

#[derive(Debug, Deserialize)]
pub struct XivDbAction {
  pub action_category: u64,
  pub action_combo: u64,
  pub action_data: u64,
  pub action_proc_status: u64,
  pub action_timeline_hit: u64,
  pub action_timeline_use: u64,
  pub can_target_dead: u64,
  pub can_target_friendly: u64,
  pub can_target_hostile: u64,
  pub can_target_party: u64,
  pub can_target_self: u64,
  pub cast_range: Option<Value>,
  pub cast_time: Option<Value>,
  pub class_name: Option<String>,
  pub classjob: Value,
  pub classjob_category: String,
  pub cost: u64,
  pub cost_cp: Option<u64>,
  pub cost_hp: Option<u64>,
  pub cost_mp: Option<u64>,
  pub cost_tp: Option<u64>,
  pub effect_range: u64,
  pub help: String,
  pub help_cns: String,
  pub help_de: String,
  pub help_en: String,
  pub help_fr: String,
  pub help_html: String,
  pub help_ja: String,
  pub icon: String,
  pub icon_hq: String,
  pub id: u64,
  pub is_in_game: u64,
  pub is_pvp: u64,
  pub is_target_area: u64,
  pub is_trait: u64,
  pub json: Option<String>,
  pub json_cns: String,
  pub json_de: Option<String>,
  pub json_en: Option<String>,
  pub json_fr: Option<String>,
  pub json_ja: Option<String>,
  pub level: u64,
  pub lodestone_id: Option<String>,
  pub lodestone_type: Option<String>,
  pub name: String,
  pub name_cns: String,
  pub name_de: String,
  pub name_en: String,
  pub name_fr: String,
  pub name_ja: String,
  pub patch: XivDbPatch,
  pub recast_time: Option<Value>,
  pub spell_group: u64,
  pub status_gain_self: u64,
  pub status_required: u64,
  #[serde(rename = "type")]
  pub type_: u64,
  pub type_name: String,
  pub upgrades: Vec<Value>,
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
