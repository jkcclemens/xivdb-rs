use serde_json::Value;

use kind::patch::XivDbPatch;

#[derive(Debug, Deserialize)]
pub struct XivDbAchievement {
    pub achievement_category: u64,
    pub category_id: u64,
    pub category_name: String,
    pub connect_post: u64,
    pub connect_pre: u64,
    pub connect_quest: u64,
    pub help: String,
    pub help_cns: String,
    pub help_de: String,
    pub help_en: String,
    pub help_fr: String,
    pub help_ja: String,
    pub icon: String,
    pub icon_hq: String,
    pub id: u64,
    pub item: Option<Value>,
    pub kind_name: String,
    pub king_id: u64,
    pub lodestone_id: String,
    pub lodestone_type: String,
    pub name: String,
    pub name_cns: String,
    pub name_de: String,
    pub name_en: String,
    pub name_fr: String,
    pub name_ja: String,
    pub order: u64,
    pub other_requirements: Vec<u64>,
    pub patch: XivDbPatch,
    pub points: u64,
    pub post_achievements: Option<Value>,
    pub pre_achievements: Option<Value>,
    pub pre_quests: Option<Value>,
    pub priority: u64,
    pub requirement_1: u64,
    pub requirement_2: u64,
    pub requirement_3: u64,
    pub requirement_4: u64,
    pub requirement_5: u64,
    pub requirement_6: u64,
    pub requirement_7: u64,
    pub requirement_8: u64,
    pub requirement_9: u64,
    pub title: Option<Value>,
    pub type: u64,
    pub type_name: String,
    pub url: String,
    pub url_api: String,
    pub url_lodestone: String,
    pub url_type: String,
    pub url_xivdb: String,
    pub url_xivdb_de: String,
    pub url_xivdb_fr: String,
    pub url_xivdb_ja: String,
    pub _cid: u64,
    pub _type: String,
}
