mod glob;
mod package_json;
mod pnpm;

use std::path::Path;
use workspace::{Workspace, WorkspacePackage};

pub struct NpmWorkspace;

impl Workspace for NpmWorkspace {
  fn package_manager(&self) -> &'static str {
    "npm"
  }

  fn packages(&self, working_dir: &Path) -> Option<Vec<WorkspacePackage>> {
    pnpm::PnpmWorkspace::try_from_dir(working_dir)?.packages(working_dir)
  }
}
