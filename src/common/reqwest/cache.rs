use color_eyre::eyre::Result;
use reqwest::{IntoUrl, blocking::Client};
use serde::de::DeserializeOwned;
use sha1::{Digest, Sha1};
use std::collections::HashMap;

type Cache<Value> = HashMap<String, Value>;

fn cache_key(url: impl AsRef<[u8]>) -> String {
  hex::encode(Sha1::digest(url))
}

pub fn get_with_cache<'a, T: IntoUrl + AsRef<[u8]> + Clone, Value: Clone + DeserializeOwned>(
  client: &Client,
  cache: &mut Cache<Value>,
  url: T,
) -> Result<Value> {
  let key = cache_key(&url);

  if let Some(response) = cache.get(&key) {
    println!("Cache hit");
    return Ok(response.clone());
  }

  println!("Cache miss");

  let response = client
    .get(url)
    .send()?
    .error_for_status()?
    .json::<Value>()?;

  cache.insert(key, response.clone());

  Ok(response)
}
