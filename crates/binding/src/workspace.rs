use nvim_oxi::conversion::{Error as ConversionError, ToObject};
use nvim_oxi::lua;
use nvim_oxi::serde::Serializer;
use nvim_oxi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceCommand {
  pub bin: String,
  pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspacePackage {
  pub name: String,
  pub root: String,
  pub commands: Vec<WorkspaceCommand>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
  pub package_manager: String,
  pub root: String,
  pub packages: Vec<WorkspacePackage>,
}

impl From<workspace::WorkspacePackage> for WorkspacePackage {
  fn from(workspace_package: workspace::WorkspacePackage) -> Self {
    Self {
      name: workspace_package.name,
      root: workspace_package.root.to_string_lossy().to_string(),
      commands: workspace_package
        .commands
        .into_iter()
        .map(Into::into)
        .collect(),
    }
  }
}

impl From<workspace::WorkspaceCommand> for WorkspaceCommand {
  fn from(workspace_command: workspace::WorkspaceCommand) -> Self {
    Self {
      bin: workspace_command.bin,
      args: workspace_command.args,
    }
  }
}

impl nvim_oxi::conversion::ToObject for Workspace {
  fn to_object(self) -> Result<Object, ConversionError> {
    self.serialize(Serializer::new()).map_err(Into::into)
  }
}

impl nvim_oxi::lua::Pushable for Workspace {
  unsafe fn push(self, lstate: *mut lua::ffi::lua_State) -> Result<std::ffi::c_int, lua::Error> {
    self
      .to_object()
      .map_err(lua::Error::push_error_from_err::<Self, _>)?
      .push(lstate)
  }
}
