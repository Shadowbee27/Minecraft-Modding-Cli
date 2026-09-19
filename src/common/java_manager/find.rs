use crate::common::java_manager::java_paths::get_possible_paths;

use super::JavaVersion;

pub fn find_javas() -> Vec<JavaVersion> {
  let _java_paths = get_possible_paths();
  unimplemented!()
}
