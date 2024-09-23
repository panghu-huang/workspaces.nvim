mod metadata;
mod package;

use metadata::CargoMetadata;
use std::path::{Path, PathBuf};
use workspace::*;

pub struct CargoWorkspace {
  metadata: CargoMetadata,
}

impl Workspace for CargoWorkspace {
  fn package_manager(&self) -> &'static str {
    "cargo"
  }

  fn workspace_root(&self) -> PathBuf {
    PathBuf::from(self.metadata.workspace_root.clone())
  }

  fn packages(&self) -> Vec<WorkspacePackage> {
    let mut packages = self
      .metadata
      .members()
      .into_iter()
      .map(|p| p.into())
      .collect::<Vec<_>>();

    if packages.len() > 1 {
      packages.push(WorkspacePackage {
        // TODO: This is a hack to get the root package name
        name: "root".to_string(),
        root: self.workspace_root(),
        commands: vec![
          WorkspaceCommand {
            bin: "cargo".to_string(),
            args: vec!["build"].into_iter().map(|s| s.to_string()).collect(),
          },
          WorkspaceCommand {
            bin: "cargo".to_string(),
            args: vec!["test"].into_iter().map(|s| s.to_string()).collect(),
          },
        ],
      })
    }

    packages
  }
}

impl CargoWorkspace {
  pub fn try_from_dir(working_dir: &Path) -> Option<CargoWorkspace> {
    CargoMetadata::from_path(working_dir)
      .map(|metadata| CargoWorkspace { metadata })
      .ok()
  }
}
