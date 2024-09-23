use serde::{Deserialize, Serialize};
use std::path::Path;
use workspace::{WorkspaceCommand, WorkspacePackage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum TargetKind {
  Lib,
  Bin,
  Example,
  Test,
  Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoPackageTarget {
  pub kind: Vec<TargetKind>,
  pub name: String,
  pub src_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoPackage {
  pub id: String,
  pub name: String,
  pub manifest_path: String,
  pub targets: Vec<CargoPackageTarget>,
}

impl CargoPackage {
  fn root(&self) -> &Path {
    let manifest_path = Path::new(&self.manifest_path);

    manifest_path.parent().unwrap()
  }

  fn available_commands(&self) -> Vec<WorkspaceCommand> {
    let mut commands = vec![self.build_command(), self.test_command()];

    let has_multiple_bins = self.has_multiple_bins();

    for target in &self.targets {
      for kind in &target.kind {
        match kind {
          TargetKind::Bin => {
            if has_multiple_bins {
              commands.push(self.run_command_with_bin_name(&target.name));
            } else {
              commands.push(self.run_command());
            }
          }
          TargetKind::Example => {
            commands.push(self.run_command_with_example_name(&target.name));
          }
          _ => {}
        }
      }
    }

    commands
  }

  fn build_command(&self) -> WorkspaceCommand {
    WorkspaceCommand {
      bin: "cargo".to_string(),
      args: ["build", "-p", &self.name]
        .iter()
        .map(|s| s.to_string())
        .collect(),
    }
  }

  fn test_command(&self) -> WorkspaceCommand {
    WorkspaceCommand {
      bin: "cargo".to_string(),
      args: ["test", "-p", &self.name]
        .iter()
        .map(|s| s.to_string())
        .collect(),
    }
  }

  fn run_command_with_bin_name(&self, bin_name: &str) -> WorkspaceCommand {
    WorkspaceCommand {
      bin: "cargo".to_string(),
      args: ["run", "-p", &self.name, "--bin", bin_name]
        .iter()
        .map(|s| s.to_string())
        .collect(),
    }
  }

  fn run_command_with_example_name(&self, example_name: &str) -> WorkspaceCommand {
    WorkspaceCommand {
      bin: "cargo".to_string(),
      args: ["run", "-p", &self.name, "--example", example_name]
        .iter()
        .map(|s| s.to_string())
        .collect(),
    }
  }

  fn run_command(&self) -> WorkspaceCommand {
    WorkspaceCommand {
      bin: "cargo".to_string(),
      args: ["run", "-p", &self.name]
        .iter()
        .map(|s| s.to_string())
        .collect(),
    }
  }

  fn has_multiple_bins(&self) -> bool {
    self
      .targets
      .iter()
      .filter(|t| t.kind.contains(&TargetKind::Bin))
      .count()
      > 1
  }
}

impl From<CargoPackage> for WorkspacePackage {
  fn from(cargo_package: CargoPackage) -> Self {
    let root = cargo_package.root().to_path_buf();
    let commands = cargo_package.available_commands();

    WorkspacePackage {
      name: cargo_package.name,
      root,
      commands,
    }
  }
}
