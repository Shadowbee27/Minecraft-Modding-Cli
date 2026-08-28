use quick_xml::de::from_str;
use reqwest::Client;
use serde::Deserialize;

// Structs were created with AI
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenMetadata {
  pub group_id: String,
  pub artifact_id: String,
  pub versioning: Versioning,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Versioning {
  pub versions: VersionsList,
  pub last_updated: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VersionsList {
  #[serde(rename = "version", default)]
  pub list: Vec<String>,
}

///
/// ```
/// get_versions("net/neoforged/neoforge","https://maven.neoforged.net/releases").unwarp();
/// ```
///
pub async fn get_versions<T: std::fmt::Display, G: std::fmt::Display>(
  group_id: T,
  base_url: G,
) -> color_eyre::Result<MavenMetadata> {
  let fetch_url = format!("{base_url}/{group_id}/maven-metadata.xml");
  dbg!(&fetch_url);
  let client = Client::new();
  let result = client.get(fetch_url).send().await?;
  let raw = result.text().await?;
  let meta: MavenMetadata = from_str(&raw)?;
  dbg!(&meta);
  Ok(meta)
}

#[cfg(test)]
mod tests {
  use tokio::runtime::Runtime;

  use super::*;

  #[test]
  fn get_forge_meta() {
    let rt = Runtime::new().unwrap();
    //
    rt.block_on(get_versions(
      "net/minecraftforge/forge",
      "https://maven.minecraftforge.net/releases",
    ))
    .unwrap();
  }

  #[test]
  fn get_neo_meta() {
    let rt = Runtime::new().unwrap();
    //
    rt.block_on(get_versions(
      "net/neoforged/neoforge",
      "https://maven.neoforged.net/releases",
    ))
    .unwrap();
  }
}
