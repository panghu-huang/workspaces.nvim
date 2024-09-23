mod workspace;

use ::workspace::Workspace;
use cargo::CargoWorkspace;
use npm::PnpmWorkspace;
use nvim_oxi::{Dictionary, Function};
use std::path::Path;

fn resolve_npm_workspace(
  working_dir: String,
  callback: Function<Option<crate::workspace::Workspace>, ()>,
) {
  let working_dir = Path::new(&working_dir);
  let Some(pnpm_workspace) = PnpmWorkspace::try_from_dir(working_dir) else {
    if let Err(err) = callback.call(None) {
      log::error!("Failed to call callback: {:?}", err);
    }
    return;
  };

  let workspace = crate::workspace::Workspace {
    package_manager: pnpm_workspace.package_manager().to_string(),
    root: pnpm_workspace
      .workspace_root()
      .to_string_lossy()
      .to_string(),
    packages: pnpm_workspace
      .packages()
      .into_iter()
      .map(Into::into)
      .collect(),
  };

  if let Err(err) = callback.call(Some(workspace)) {
    log::error!("Failed to call callback: {:?}", err);
  }
}

fn resolve_cargo_workspace(
  working_dir: String,
  callback: Function<Option<crate::workspace::Workspace>, ()>,
) {
  let working_dir = Path::new(&working_dir);
  let Some(cargo_workspace) = CargoWorkspace::try_from_dir(working_dir) else {
    if let Err(err) = callback.call(None) {
      log::error!("Failed to call callback: {:?}", err);
    }
    return;
  };
  let workspace = crate::workspace::Workspace {
    package_manager: cargo_workspace.package_manager().to_string(),
    root: cargo_workspace
      .workspace_root()
      .to_string_lossy()
      .to_string(),
    packages: cargo_workspace
      .packages()
      .into_iter()
      .map(Into::into)
      .collect(),
  };

  if let Err(err) = callback.call(Some(workspace)) {
    log::error!("Failed to call callback: {:?}", err);
  }
}

#[nvim_oxi::plugin]
fn workspace_binding() -> Dictionary {
  let resolve_cargo_workspace = Function::from_fn(
    |(working_dir, callback): (String, Function<Option<crate::workspace::Workspace>, ()>)| {
      resolve_cargo_workspace(working_dir, callback);
    },
  );

  let resolve_npm_workspace = Function::from_fn(
    |(working_dir, callback): (String, Function<Option<crate::workspace::Workspace>, ()>)| {
      resolve_npm_workspace(working_dir, callback);
    },
  );

  Dictionary::from_iter([
    ("resolve_cargo_workspace", resolve_cargo_workspace),
    ("resolve_npm_workspace", resolve_npm_workspace),
  ])
}
