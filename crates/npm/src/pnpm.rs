use super::glob::find_package_json_paths;
use super::package_json::PackageJson;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use workspace::{Workspace, WorkspaceCommand, WorkspacePackage};

#[derive(Debug, Clone, Deserialize)]
struct WorkspaceConfig {
  packages: Vec<String>,
}

pub struct PnpmWorkspace {
  root: PathBuf,
  config: Option<WorkspaceConfig>,
}

impl Workspace for PnpmWorkspace {
  fn package_manager(&self) -> &'static str {
    "pnpm"
  }

  fn workspace_root(&self) -> PathBuf {
    self.root.clone()
  }

  fn packages(&self) -> Vec<WorkspacePackage> {
    let mut packages = self.resolve_root_package().into_iter().collect::<Vec<_>>();

    if let Some(config) = &self.config {
      for package_pattern in &config.packages {
        let package_paths = match find_package_json_paths(&self.root, package_pattern) {
          Ok(paths) => paths,
          Err(_) => continue,
        };

        for package_path in package_paths {
          match self.resolve_workspace_package(package_path) {
            Some(package) => packages.push(package),
            None => continue,
          }
        }
      }
    }

    packages
  }
}

impl PnpmWorkspace {
  fn resolve_workspace_package(&self, package_path: PathBuf) -> Option<WorkspacePackage> {
    let package_json = PackageJson::try_from_file(&package_path)?;
    let package_name = package_json.name?;

    let commands = package_json
      .scripts
      .unwrap_or_default()
      .keys()
      .map(|name| WorkspaceCommand {
        bin: "pnpm".to_string(),
        args: vec![
          "--filter".to_string(),
          package_name.clone(),
          "run".to_string(),
          name.to_string(),
        ],
      })
      .collect();

    let package = WorkspacePackage {
      root: package_path.parent()?.to_path_buf(),
      name: package_name,
      commands,
    };

    Some(package)
  }

  fn resolve_root_package(&self) -> Option<WorkspacePackage> {
    let root_package = self.root.join("package.json");
    let package_json = PackageJson::try_from_file(&root_package)?;

    let package_name = package_json.name?;
    let commands = package_json
      .scripts
      .unwrap_or_default()
      .keys()
      .map(|name| WorkspaceCommand {
        bin: "pnpm".to_string(),
        args: vec!["run".to_string(), name.to_string()],
      })
      .collect();

    let package = WorkspacePackage {
      root: self.root.clone(),
      name: package_name,
      commands,
    };

    Some(package)
  }
}

impl PnpmWorkspace {
  pub fn try_from_dir(working_dir: &Path) -> Option<PnpmWorkspace> {
    if working_dir.is_file() {
      let parent = working_dir.parent()?;
      return PnpmWorkspace::try_from_dir(parent);
    }

    if working_dir.join("pnpm-lock.yaml").exists() {
      let config = PnpmWorkspace::resolve_workspace_config(working_dir);

      return Some(PnpmWorkspace {
        root: working_dir.to_path_buf(),
        config,
      });
    }

    let parent = working_dir.parent()?;

    PnpmWorkspace::try_from_dir(parent)
  }

  fn resolve_workspace_config(root_dir: &Path) -> Option<WorkspaceConfig> {
    let file_path = root_dir.join("pnpm-workspace.yaml");
    let file = std::fs::File::open(&file_path).ok()?;
    let config: Option<WorkspaceConfig> = serde_yaml::from_reader(file).ok()?;

    config
  }
}
