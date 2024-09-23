use super::package::CargoPackage;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoMetadata {
  pub workspace_root: String,
  pub workspace_members: Vec<String>,
  pub packages: Vec<CargoPackage>,
}

impl CargoMetadata {
  pub fn members(&self) -> Vec<CargoPackage> {
    let mut members = vec![];

    for member in &self.workspace_members {
      if let Some(package) = self.packages.iter().find(|p| p.id == *member) {
        members.push(package.clone());
      }
    }

    members
  }
}

impl CargoMetadata {
  pub fn from_path(path: &Path) -> anyhow::Result<Self> {
    if path.is_file() {
      let path = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("invalid path"))?;

      return Self::from_path(path);
    }

    let output = std::process::Command::new("cargo")
      .arg("metadata")
      .arg("--format-version=1")
      .current_dir(path)
      .output()?;

    let stdout = String::from_utf8(output.stdout)?;

    Self::from_json(&stdout)
  }

  pub fn from_json(json: &str) -> anyhow::Result<Self> {
    Ok(serde_json::from_str(json)?)
  }
}
