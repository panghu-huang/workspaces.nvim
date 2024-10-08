use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceCommand {
  pub bin: String,
  pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspacePackage {
  pub name: String,
  pub root: PathBuf,
  pub commands: Vec<WorkspaceCommand>,
}

pub trait Workspace {
  fn package_manager(&self) -> &'static str;
  fn workspace_root(&self) -> PathBuf;
  fn packages(&self) -> Vec<WorkspacePackage>;
}
