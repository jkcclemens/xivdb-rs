use serde_json::Value;

use kind::patch::XivDbPatch;

#[derive(Debug, Deserialize)]
pub struct XivDbEmote {
  pub command: String,
  pub emote_category: String,
  pub icon: String,
  pub icon_hq: String,
  pub id: u64,
  pub lodestone_id: Option<String>,
  pub lodestone_type: Option<String>,
  pub log_message_targeted: u64,
  pub log_message_untargeted: u64,
  pub name: String,
  pub name_cns: String,
  pub name_de: String,
  pub name_en: String,
  pub name_fr: String,
  pub name_ja: String,
  pub patch: XivDbPatch,
  pub text_command: XivdbEmoteTextCommand,
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

#[derive(Debug, Deserialize)]
pub struct XivDbEmoteTextCommand {
  pub command1: String,
  pub command2: String,
  pub command3: String,
  pub command4: String,
  pub help: String,
  pub help_html: String,
}
