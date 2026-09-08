use color_eyre::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs::{DirEntry, read_dir};

use crate::{
  config::mod_index::{ModMeta, ModsIndex},
  mod_source::{ModSource, modrinth::api::get_version_from_hash},
  tui::dialogs::mod_source::preferred_mod_source,
};

pub fn index_mods(index: &mut ModsIndex) -> Result<()> {
  dbg!(&index);
  let files = read_dir(&index.path)?;

  let mut jars: Vec<DirEntry> = Vec::new();

  for file in files {
    if let Ok(f) = file
      && f.path().extension().unwrap_or_default().to_str() == Some("jar")
    {
      jars.push(f);
    }
  }

  if jars.is_empty() {
    return Ok(());
  }

  let mod_source = preferred_mod_source()?;

  let mod_meta_vec: Vec<ModMeta> = jars
    .par_iter()
    .map(|j| {
      let mut mod_meta = ModMeta::default();
      let path = j.path();
      mod_meta.hash = get_sha256(path).unwrap();

      if mod_source == ModSource::MODRINTH {
        get_version_from_hash(&mod_meta.hash.clone(), &mut mod_meta).unwrap();
      }

      mod_meta
    })
    .collect();

  index.mod_meta = Some(mod_meta_vec);
  Ok(())
}

fn get_sha256<T: std::marker::Send + std::convert::AsRef<std::path::Path>>(
  path: T,
) -> Result<String> {
  Ok(sha256::try_digest(path)?)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hash() {
    assert_eq!(
      get_sha256("./LICENSE").unwrap(),
      "8dde21394cabe75d8dffaff8d3c9f28f9b495c05dbfc296693dbaecf57bce46f"
    );
  }
}
