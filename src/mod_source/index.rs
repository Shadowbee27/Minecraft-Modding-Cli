use color_eyre::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use sha1::{Digest, Sha1};
use std::{
  fs::{DirEntry, File, read_dir},
  io::{BufReader, Read},
  time::Instant,
};

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

  let time = Instant::now();

  let mod_meta_vec: Vec<ModMeta> = jars
    .par_iter()
    .map(|j| {
      let mut mod_meta = ModMeta::default();
      let path = j.path();
      mod_meta.hash = get_sha256(path).unwrap();

      if mod_source == ModSource::MODRINTH {
        get_version_from_hash(mod_meta.hash.clone(), &mut mod_meta).unwrap();
      }

      mod_meta
    })
    .collect();

  dbg!(time.elapsed());

  index.mod_meta = Some(mod_meta_vec);
  Ok(())
}

fn get_sha256<T: std::marker::Send + std::convert::AsRef<std::path::Path>>(
  path: T,
) -> Result<String> {
  let file = File::open(path)?;
  let mut reader = BufReader::new(file);

  let mut hasher = Sha1::new();
  let mut buffer = [0u8; 8192];

  loop {
    let bytes_read = reader.read(&mut buffer)?;

    if bytes_read == 0 {
      break;
    }

    hasher.update(&buffer[..bytes_read]);
  }

  let hash = hasher.finalize();
  Ok(hex::encode(hash))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hash() {
    assert_eq!(
      get_sha256("./LICENSE").unwrap(),
      "51fbad96d4ea2d0c5302e0f276e46f973b06eea0"
    );
  }
}
