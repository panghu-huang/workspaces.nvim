mod metadata;
mod package;

use metadata::CargoMetadata;
use std::path::Path;
use workspace::*;

#[derive(Default)]
pub struct CargoWorkspace;

impl Workspace for CargoWorkspace {
  fn package_manager(&self) -> &'static str {
    "cargo"
  }

  fn packages(&self, working_dir: &std::path::Path) -> Option<Vec<WorkspacePackage>> {
    let metadata = self
      .get_metadata(working_dir)
      .map_err(|e| {
        log::error!("Failed to get cargo metadata: {}", e);
      })
      .ok()?;

    Some(
      metadata
        .members()
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<_>>(),
    )
  }
}

impl CargoWorkspace {
  fn get_metadata(&self, root: &Path) -> anyhow::Result<CargoMetadata> {
    CargoMetadata::from_path(root)
  }
}
