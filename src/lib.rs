#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate error_chain;
extern crate hyper;
extern crate hyper_rustls;
extern crate url;

pub mod kind;
pub mod error;

use std::collections::HashMap;

use hyper::Client;
use hyper::net::HttpsConnector;

use url::Url;

use kind::prelude::*;
use kind::search::prelude::*;
use error::*;

const API_BASE_URL: &'static str = "https://api.xivdb.com";

pub const DEFAULT_PARAMS: &'static [(String, String)] = &[];

pub struct XivDb {
  client: Client
}

impl Default for XivDb {
  fn default() -> XivDb { XivDb::new() }
}

impl XivDb {
  pub fn new() -> XivDb {
    XivDb {
      client: Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()))
    }
  }

  pub fn search<K: AsRef<str>, V: AsRef<str>>(&self, string: &str, other_params: &[(K, V)]) -> Result<XivDbSearchResult> {
    let other_params: HashMap<String, String> = other_params.iter()
      .map(|&(ref k, ref v)| (k.as_ref().to_string(), v.as_ref().to_string()))
      .collect();
    let mut url = Url::parse(&format!("{}/search", API_BASE_URL)).chain_err(|| "could not parse API base url")?;
    url.query_pairs_mut()
      .append_pair("string", string)
      .extend_pairs(other_params);
    let res = self.client.get(url).send().chain_err(|| "error downloading from xivdb's api")?;
    serde_json::from_reader(res).chain_err(|| "error deserializing downloaded data")
  }

  pub fn character(&self, id: u64) -> Result<XivDbCharacter> {
    let url = format!("{}/character/{}", API_BASE_URL, id);
    let res = self.client.get(&url).send().chain_err(|| "error downloading from xivdb's api")?;
    serde_json::from_reader(res).chain_err(|| "error deserializing downloaded data")
  }
}
