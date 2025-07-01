use std::{os::windows::process::CommandExt, process::Command};

use crate::{ahqdb::{ps1::install::to_pwsh_string, AHQDBError}, zip::link::Type};

pub mod install;

pub fn run_is_installed_ps1(
  script: &str,
  ahqdb_dir: &str,
  ty: &Type
) -> Result<bool, AHQDBError> {
  let script_path = to_pwsh_string(script);

  let command = match ty {
    Type::AllUsers => format!(r#"-ExecutionPolicy Bypass -Command "& {{$env:EXECUTION_MODE='AdminMode'; (cat '{}' | iex)}}""#, script_path),
    Type::CurrentUser => format!(r#"-ExecutionPolicy Bypass -Command "& {{$env:EXECUTION_MODE='UserMode'; (cat '{}' | iex)}}""#, script_path)
  };

  let code = Command::new("powershell.exe")
    .raw_arg(command)
    .creation_flags(0x08000000)
    .current_dir(ahqdb_dir)
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