use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct XivDbCharacter {
  pub lodestone_id: u64,
  pub name: String,
  pub server: String,
  pub avatar: String,
  pub added: String,
  pub last_updated: String,
  pub last_synced: String,
  pub data_last_changed: String,
  pub data_hash: String,
  pub update_count: u64,
  pub achievements_last_updated: String,
  pub achievements_last_changed: Option<String>,
  pub achievements_public: u64,
  pub achievements_score_reborn: u64,
  pub achievements_score_legacy: u64,
  pub achievements_score_reborn_total: u64,
  pub achievements_score_legacy_total: u64,
  pub deleted: String,
  pub priority: u64,
  pub patch: u64,
  pub data: XivDbCharacterData,
  pub grand_companies: Option<XivDbCharacterGrandCompanies>,
  pub portrait: String,
  pub last_active: String,
  pub url: String,
  pub url_api: String,
  pub url_xivdb: String,
  pub url_lodestone: String,
  pub url_type: String,
  pub extras: XivDbCharacterExtras
}

#[derive(Debug, Deserialize)]
pub struct XivDbCharacterData {
  pub id: String,
  pub name: String,
  pub server: String,
  pub title: Option<String>,
  pub avatar: String,
  pub portrait: String,
  pub biography: String,
  pub race: String,
  pub clan: String,
  pub gender: String,
  pub nameday: String,
  pub guardian: XivDbCharacterGuardian,
  pub city: XivDbCharacterCity,
  pub grand_company: Option<XivDbCharacterGrandCompany>,
  #[serde(rename = "classjobs")]
  pub class_jobs: Value,
  pub mounts: Value,
  pub minions: Value,
  pub active_class: XivDbCharacterActiveClass
}

#[derive(Debug, Deserialize)]
pub struct XivDbCharacterActiveClass {
  pub role: XivDbCharacterActiveClassRole,
  pub progress: XivDbCharacterActiveClassProgress
}

#[derive(Debug, Deserialize)]
pub struct XivDbCharacterActiveClassRole {
  pub id: u64,
  pub name: String,
  pub abbr: String,
  pub is_job: u64,
  pub classjob_parent: Option<u64>,
  pub icon: String,
  pub patch: u64,
}

#[derive(Debug, Deserialize)]
pub struct XivDbCharacterActiveClassProgress {
  pub name: String,
  pub level: u64,
  pub exp: XivDbCharacterActiveClassProgressExp,
  pub id: u64,
  pub data: XivDbCharacterActiveClassRole,
  pub level_togo: u64,
  pub level_percent: f32
}

#[derive(Debug, Deserialize)]
pub struct XivDbCharacterActiveClassProgressExp {
  pub current: u64,
  pub max: u64,
  pub at_cap: bool,
  pub total_gained: u64,
  pub total_max: u64,
  pub total_togo: u64,
  pub percent: f32,
  pub total_percent: f32
}

#[derive(Debug, Deserialize)]
pub struct XivDbCharacterGuardian {
  pub icon: String,
  pub name: String
}

#[derive(Debug, Deserialize)]
pub struct XivDbCharacterCity {
  pub icon: String,
  pub name: String
}

#[derive(Debug, Deserialize)]
pub struct XivDbCharacterGrandCompany {
  pub icon: String,
  pub name: String,
  pub rank: String
}

#[derive(Debug, Deserialize)]
pub struct XivDbCharacterGrandCompanies {
  #[serde(default)]
  pub order_of_the_twin_adder: Option<XivDbCharacterGrandCompanyStanding>,
  pub maelstrom: Option<XivDbCharacterGrandCompanyStanding>,
  pub immortal_flames: Option<XivDbCharacterGrandCompanyStanding>
}

#[derive(Debug, Deserialize)]
pub struct XivDbCharacterGrandCompanyStanding {
  pub icon: String,
  pub name: String,
  pub rank: String,
  pub time: String
}

#[derive(Debug, Deserialize)]
pub struct XivDbCharacterExtras {
  pub mounts: XivDbCharacterExtrasData,
  pub minions: XivDbCharacterExtrasData
}

#[derive(Debug, Deserialize)]
pub struct XivDbCharacterExtrasData {
  pub obtained: u64,
  pub total: u64,
  pub percent: f32
}
