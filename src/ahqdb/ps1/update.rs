use std::{os::windows::process::CommandExt, process::Command};

use crate::{ahqdb::{ps1::install::to_pwsh_string, AHQDBError}, zip::link::Type};

pub fn run_update_ps1(
  script: &str,
  ahqdb_old_dir: &str,
  ahqdb_new_dir: &str,
  build: &str,
  ty: &Type
) -> Result<bool, AHQDBError> {
  let script_path = to_pwsh_string(script);

  let new = to_pwsh_string(ahqdb_new_dir);
  let old = to_pwsh_string(ahqdb_old_dir);

  let command = match ty {
    Type::AllUsers => format!(r#"-ExecutionPolicy Bypass -Command "& {{$env:EXECUTION_MODE='AdminMode'; $env:AHQDB_OLD_INSTALL_DIR='{old}'; $env:AHQDB_NEW_INSTALL_DIR='{new}'; $env:OLD_BUILD='{build}'; (cat '{}' | iex)}}""#, script_path, build = build),
    Type::CurrentUser => format!(r#"-ExecutionPolicy Bypass -Command "& {{$env:EXECUTION_MODE='UserMode'; $env:AHQDB_OLD_INSTALL_DIR='{old}'; $env:AHQDB_NEW_INSTALL_DIR='{new}'; $env:OLD_BUILD='{build}'; (cat '{}' | iex)}}""#, script_path, build = build)
  };

  let code = Command::new("powershell.exe")
    .raw_arg(command)
    .creation_flags(0x08000000)
    .current_dir(ahqdb_new_dir)
    .spawn()?
    .wait()
    .unwrap()
    .code()
    .unwrap_or(-100);

  match code {
    0 => Ok(true),
    1 => Ok(false),
    code => Err(AHQDBError::InvalidOutCode(code)),
  }
}