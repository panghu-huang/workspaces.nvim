use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct PackageJson {
  pub name: Option<String>,
  pub scripts: Option<HashMap<String, String>>,
}

impl PackageJson {
  pub fn try_from_file(file_path: &Path) -> Option<PackageJson> {
    let file = std::fs::File::open(file_path).ok()?;
    let package_json: PackageJson = serde_json::from_reader(file).ok()?;

    Some(package_json)
  }
}
