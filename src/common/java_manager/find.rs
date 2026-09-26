use color_eyre::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::PathBuf;

use crate::common::java_manager::java_paths::{FILTERS, JavaPath, get_possible_paths};

use super::JavaVersion;

pub fn find_javas() -> Result<Vec<JavaVersion>> {
  let java_paths = get_possible_paths();
  let _check_paths = scan(java_paths)?;
  unimplemented!()
}

pub fn scan(possible_paths: Vec<JavaPath>) -> Result<Vec<PathBuf>> {
  let mut existing_paths = Vec::new();

  let todo: Vec<Vec<PathBuf>> = possible_paths
    .par_iter()
    .map(|p| FILTERS::run(p.clone()).unwrap())
    .collect();

  for mut vector in todo {
    existing_paths.append(&mut vector);
  }
  Ok(existing_paths)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn java_paths() {
    let java_paths = get_possible_paths();
    let check_paths = scan(java_paths).unwrap();
    dbg!(check_paths);
  }
}
