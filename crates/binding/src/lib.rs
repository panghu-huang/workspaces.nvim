mod package;

use nvim_oxi::{Dictionary, Function};
use std::path::Path;
use workspace::Workspace;

fn resolve_npm_workspace(working_dir: String) -> Option<Vec<workspace::WorkspacePackage>> {
  let working_dir = Path::new(&working_dir);
  let npm_workspace = npm::NpmWorkspace;

  npm_workspace.packages(working_dir)
}

fn resolve_cargo_workspace(working_dir: String) -> Option<Vec<workspace::WorkspacePackage>> {
  let working_dir = Path::new(&working_dir);
  let cargo_workspace = cargo::CargoWorkspace;

  cargo_workspace.packages(working_dir)
}

#[nvim_oxi::plugin]
fn workspace_binding() -> Dictionary {
  let resolve_cargo_workspace = Function::from_fn(
    |working_dir: String| -> Option<Vec<package::WorkspacePackage>> {
      resolve_cargo_workspace(working_dir)
        .map(|packages| packages.into_iter().map(Into::into).collect())
    },
  );

  let resolve_npm_workspace = Function::from_fn(
    |working_dir: String| -> Option<Vec<package::WorkspacePackage>> {
      resolve_npm_workspace(working_dir)
        .map(|packages| packages.into_iter().map(Into::into).collect())
    },
  );

  Dictionary::from_iter([
    ("resolve_cargo_workspace", resolve_cargo_workspace),
    ("resolve_npm_workspace", resolve_npm_workspace),
  ])
}
