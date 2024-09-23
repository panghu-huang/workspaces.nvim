use cargo::CargoWorkspace;
use workspace::{Workspace, WorkspaceCommand, WorkspacePackage};

fn command_to_string(command: &WorkspaceCommand) -> String {
  format!("{} {}", command.bin, command.args.join(" "))
}

fn find_package_by_name<'a>(
  packages: &'a [WorkspacePackage],
  name: &'a str,
) -> Option<&'a WorkspacePackage> {
  packages.iter().find(|p| p.name == name)
}

#[test]
fn test_cargo_workspace() {
  let root = std::env::current_dir().unwrap();
  let workspace = CargoWorkspace::try_from_dir(&root).unwrap();

  let packages = workspace.packages();

  assert_eq!(packages.len(), 5);

  let package = find_package_by_name(&packages, "cargo").unwrap();

  assert_eq!(package.name, "cargo");

  assert_eq!(package.commands.len(), 2);

  assert_eq!(
    command_to_string(&package.commands[0]),
    "cargo build -p cargo"
  );

  assert_eq!(
    command_to_string(&package.commands[1]),
    "cargo test -p cargo"
  );
}
