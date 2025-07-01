use std::{io::Result, os::windows::process::CommandExt, process::Command};
use crate::zip::link::Type;

pub fn run_install_ps1(
  script: &str,
  ahqdb_dir: &str,
  ty: &Type
) -> Result<()> {
  let parsed_path = to_pwsh_string(ahqdb_dir);

  let script_path = to_pwsh_string(script);

  let command = match ty {
    Type::AllUsers => format!(r#"-ExecutionPolicy Bypass -Command "& {{$env:AHQDB_INSTALL_DIR='{}'; $env:EXECUTION_MODE='AdminMode'; (cat '{}' | iex)}}""#, parsed_path, script_path),
    Type::CurrentUser => format!(r#"-ExecutionPolicy Bypass -Command "& {{$env:AHQDB_INSTALL_DIR='{}'; $env:EXECUTION_MODE='UserMode'; (cat '{}' | iex)}}""#, parsed_path, script_path)
  };

  Command::new("powershell.exe")
    .raw_arg(command)
    //.creation_flags(0x08000000)
    .current_dir(ahqdb_dir)
    .spawn()?
    .wait()
    .unwrap();

  Ok(())
}

pub fn run_uninstall_ps1(
  script: &str,
  ahqdb_dir: &str,
  ty: &Type
) -> Result<()> {
  let parsed_path = to_pwsh_string(ahqdb_dir);

  let script_path = to_pwsh_string(script);

  let command = match ty {
    Type::AllUsers => format!(r#"-ExecutionPolicy Bypass -Command "& {{$env:AHQDB_INSTALL_DIR='{}'; $env:EXECUTION_MODE='AdminMode'; (cat '{}' | iex)}}""#, parsed_path, script_path),
    Type::CurrentUser => format!(r#"-ExecutionPolicy Bypass -Command "& {{$env:AHQDB_INSTALL_DIR='{}'; $env:EXECUTION_MODE='UserMode'; (cat '{}' | iex)}}""#, parsed_path, script_path)
  };

  Command::new("powershell.exe")
    .raw_arg(command)
    .creation_flags(0x08000000)
    .current_dir(ahqdb_dir)
    .spawn()?
    .wait()
    .unwrap();

  Ok(())
}

pub fn to_pwsh_string(dat: &str) -> String {
  let mut data = String::new();

  for r#char in dat.chars().into_iter() {
    match r#char {
      '"' => data.push_str("`\""),
      '$' => data.push_str("`$"),
      '`' => data.push_str("``"),
      '\n' => data.push_str("`n"),
      '\r' => data.push_str("`r"),
      '\t' => data.push_str("`t"),
      e => data.push(e),
    }
  }

  data
}