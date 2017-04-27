use serde_json::Value;

use kind::patch::XivDbPatch;

#[derive(Debug, Deserialize)]
pub struct XivDbItem {
  pub achievements: Option<Value>,
  pub action: u64,
  pub aetherial_reduce: u64,
  pub attributes_base: XivDbItemAttributes,
  pub attributes_params: Vec<Value>,
  pub attributes_params_special: Vec<Value>,
  pub base_param_modifier: u64,
  pub bonus_name: Option<String>,
  pub can_be_hq: u64,
  pub category_name: String,
  pub classjob_category: Option<Value>,
  pub classjob_repair: Option<Value>,
  pub classjob_use: u64,
  pub classjobs: Vec<Value>,
  pub color: String,
  pub connect_achievement: u64,
  pub connect_craftable: u64,
  pub connect_enemy_drop: u64,
  pub connect_gathering: u64,
  pub connect_instance: u64,
  pub connect_instance_chest: u64,
  pub connect_instance_reward: u64,
  pub connect_leve: u64,
  pub connect_quest_reward: u64,
  pub connect_recipe: u64,
  pub connect_shop: u64,
  pub connect_specialshop_cost_1: u64,
  pub connect_specialshop_cost_2: u64,
  pub connect_specialshop_cost_3: u64,
  pub connect_specialshop_receive_1: u64,
  pub connect_specialshop_receive_2: u64,
  pub connected: bool,
  pub cooldown_seconds: u64,
  pub craftable: Option<Value>,
  pub desynthesize: u64,
  pub enemies: Option<Value>,
  pub equip_slot_category: u64,
  pub equippable_by_gender_f: u64,
  pub equippable_by_gender_m: u64,
  pub equippable_by_race_aura: u64,
  pub equippable_by_race_elezen: u64,
  pub equippable_by_race_hyur: u64,
  pub equippable_by_race_lalafell: u64,
  pub equippable_by_race_miqote: u64,
  pub equippable_by_race_roegadyn: u64,
  pub gathering: Option<Value>,
  pub gc_turn_in: u64,
  pub grand_company: u64,
  pub help: String,
  pub help_cns: String,
  pub help_de: String,
  pub help_en: String,
  pub help_fr: String,
  pub help_ja: String,
  pub icon: String,
  pub icon_hq: Option<String>,
  pub icon_lodestone: String,
  pub icon_lodestone_hq: String,
  pub id: u64,
  pub instances: Option<Value>,
  pub is_collectable: u64,
  pub is_convertible: u64,
  pub is_crest_worthy: u64,
  pub is_dated: u64,
  pub is_desynthesizable: u64,
  pub is_dyeable: u64,
  pub is_indisposable: u64,
  pub is_legacy: u64,
  pub is_projectable: u64,
  pub is_pvp: u64,
  pub is_reducible: u64,
  pub is_unique: u64,
  pub is_untradable: u64,
  pub item_action: u64,
  pub item_duration: u64,
  pub item_glamour: Option<Value>,
  pub item_repair: Option<Value>,
  pub item_search_category: u64,
  pub item_series: u64,
  pub item_special_bonus: u64,
  pub item_ui_category: u64,
  pub item_ui_kind: u64,
  pub kind_name: String,
  pub level_equip: u64,
  pub level_item: u64,
  pub leves: Option<Value>,
  pub lodestone_id: String,
  pub lodestone_type: String,
  pub materia_slot_count: u64,
  pub materialize_type: u64,
  pub model_main: String,
  pub model_sub: String,
  pub name: String,
  pub name_cns: String,
  pub name_de: String,
  pub name_en: String,
  pub name_fr: String,
  pub name_ja: String,
  pub noun: Option<String>,
  pub noun_cns: String,
  pub noun_de: Option<String>,
  pub noun_en: Option<String>,
  pub noun_fr: Option<String>,
  pub noun_ja: Option<String>,
  pub parsed_lodestone: u64,
  pub parsed_lodestone_time: String,
  pub patch: XivDbPatch,
  pub plural: Option<String>,
  pub plural_cns: String,
  pub plural_de: Option<String>,
  pub plural_en: Option<String>,
  pub plural_fr: Option<String>,
  pub plural_ja: Option<String>,
  pub price_high: u64,
  pub price_low: u64,
  pub price_mid: u64,
  pub price_sell: u64,
  pub price_sell_hq: u64,
  pub pvp_rank: u64,
  pub quests: Option<Value>,
  pub rarity: u64,
  pub recipes: Vec<Value>,
  pub reducible_classjob: Option<Value>,
  pub reducible_level: u64,
  pub salvage: u64,
  pub series_name: Option<String>,
  pub shops: Option<Value>,
  pub slot_equip: u64,
  pub slot_name: Option<String>,
  pub sort_key: u64,
  pub special_shops_currency: Option<Value>,
  pub special_shops_obtain: Option<Value>,
  pub stack_size: u64,
  pub stain: u64,
  pub starts_with_vowel: u64,
  pub updated: String,
  pub url: String,
  pub url_api: String,
  pub url_lodestone: String,
  pub url_type: String,
  pub url_xivdb: String,
  pub url_xivdb_de: String,
  pub url_xivdb_fr: String,
  pub url_xivdb_ja: String,
  pub _cid: u64,
  pub _type: String
}

#[derive(Debug, Deserialize)]
pub struct XivDbItemAttributes {
  pub auto_attack: u64,
  pub auto_attack_hq: u64,
  pub block_rate: u64,
  pub block_rate_hq: u64,
  pub block_strength: u64,
  pub block_strength_hq: u64,
  pub damage: u64,
  pub damage_hq: u64,
  pub defense: u64,
  pub defense_hq: u64,
  pub delay: u64,
  pub delay_hq: u64,
  pub dps: u64,
  pub dps_hq: u64,
  pub id: u64,
  pub magic_damage: u64,
  pub magic_damage_hq: u64,
  pub magic_defense: u64,
  pub magic_defense_hq: u64,
  pub patch: u64
}
