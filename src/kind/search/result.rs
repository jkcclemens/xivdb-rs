use kind::search::prelude::*;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct XivDbSearchResult {
  pub items: Option<XivDbCategory>,
  pub recipes: Option<XivDbCategory>,
  pub quests: Option<XivDbCategory>,
  pub actions: Option<XivDbCategory>,
  pub instances: Option<XivDbCategory>,
  pub achievements: Option<XivDbCategory>,
  pub fates: Option<XivDbCategory>,
  pub leves: Option<XivDbCategory>,
  pub places: Option<XivDbCategory>,
  pub gathering: Option<XivDbCategory>,
  pub npcs: Option<XivDbCategory>,
  pub enemies: Option<XivDbCategory>,
  pub emotes: Option<XivDbCategory>,
  pub status: Option<XivDbCategory>,
  pub titles: Option<XivDbCategory>,
  pub minions: Option<XivDbCategory>,
  pub mounts: Option<XivDbCategory>,
  pub weather: Option<XivDbCategory>,
  pub characters: Option<XivDbCategory>,
}
