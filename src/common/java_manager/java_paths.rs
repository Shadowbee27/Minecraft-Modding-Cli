/*
 * The code is somewhat copied from https://github.com/PrismLauncher/PrismLauncher/blob/release-11.x/launcher/java/JavaUtils.cpp
 */

use std::env;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JavaPath {
  pub path: PathBuf,
  pub java_type: JavaType,
  pub filter: FILTERS,
}

impl JavaPath {
  pub fn from<T>(path: T, javatype: JavaType, filter: FILTERS) -> JavaPath
  where
    T: Into<PathBuf>,
  {
    JavaPath {
      path: path.into(),
      java_type: javatype,
      filter: filter,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FILTERS {
  GENTOO,
  AOSC,
  STANDARD,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JavaType {
  BIN,
  SCANPATH,
  #[cfg(target_os = "windows")]
  REGISTRY,
}

#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
pub fn get_possible_paths<'a>() -> Vec<JavaPath> {
  let home = env::home_dir().unwrap();
  let mut paths = Vec::new();

  paths.append(&mut vec![
    // javas downloaded by IntelliJ
    JavaPath::from(home.join(".jdks"), JavaType::SCANPATH, FILTERS::STANDARD),
    // javas downloaded by gradle (toolchains)
    JavaPath::from(
      home.join(".gradle").join("jdks"),
      JavaType::SCANPATH,
      FILTERS::STANDARD,
    ),
    // javas downloaded by asdf
    JavaPath::from(
      home.join(".asdf").join("installs").join("java"),
      JavaType::SCANPATH,
      FILTERS::STANDARD,
    ),
    // Javas downloaded by sdkman
    JavaPath::from(
      home.join(".sdkman").join("candidates").join("java"),
      JavaType::SCANPATH,
      FILTERS::STANDARD,
    ),
  ]);

  #[cfg(target_os = "linux")]
  paths.append(&mut vec![
    // oracle RPMs
    JavaPath::from("/usr/java", JavaType::SCANPATH, FILTERS::STANDARD),
    // general locations used by distro packaging
    JavaPath::from("/usr/lib/jvm", JavaType::SCANPATH, FILTERS::STANDARD),
    JavaPath::from("/usr/lib32/jvm", JavaType::SCANPATH, FILTERS::STANDARD),
    JavaPath::from("/usr/lib64/jvm", JavaType::SCANPATH, FILTERS::STANDARD),
    // Gentoo's locations for openjdk and openjdk-bin respectively
    JavaPath::from("/usr/lib", JavaType::SCANPATH, FILTERS::GENTOO),
    JavaPath::from("/usr/lib64", JavaType::SCANPATH, FILTERS::GENTOO),
    JavaPath::from("/opt", JavaType::SCANPATH, FILTERS::GENTOO),
    // AOSC OS's locations for openjdk
    JavaPath::from("/usr/lib", JavaType::SCANPATH, FILTERS::AOSC),
    // manually installed JDKs in /opt
    JavaPath::from("/opt/jdk", JavaType::SCANPATH, FILTERS::STANDARD),
    JavaPath::from("/opt/jdks", JavaType::SCANPATH, FILTERS::STANDARD),
    // IBM Semeru Certified Edition
    JavaPath::from("/opt/ibm", JavaType::SCANPATH, FILTERS::STANDARD),
  ]);

  #[cfg(any(target_os = "freebsd", target_os = "openbsd"))]
  paths.append(&mut vec![JavaPath::from(
    "/usr/local",
    JavaType::SCANPATH,
    FILTERS::STANDARD,
  )]);

  paths.sort();
  paths
}

#[cfg(target_os = "macos")]
pub fn get_possible_paths() -> Vec<JavaPath> {
  vec![]
}

#[cfg(target_os = "windows")]
pub fn get_possible_paths() -> Vec<JavaPath> {
  vec![]
}
