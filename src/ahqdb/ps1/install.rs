use std::{io::Result, os::windows::process::CommandExt, process::Command};

use crate::{common::run_as_admin, zip::link::Type};

pub fn run_install_ps1(
  script: &str,
  ahqdb_dir: &str,
  ty: Type
) -> Result<()> {
  let parsed_path = to_pwsh_string(ahqdb_dir);

  let script_path = to_pwsh_string(script);

  match ty {
    Type::AllUsers => {
      let command = format!(r#"-NoExit -ExecutionPolicy Bypass -Command "& {{$env:AHQDB_INSTALL_DIR=`"{}`"; $env:EXECUTION_MODE=`"AdminMode`"; (cat `"{}`" | iex)}}""#, parsed_path, script_path);

      run_as_admin("powershell.exe", Some(&command))?;
    }
    Type::CurrentUser => {
      let command = format!(r#"-NoExit -ExecutionPolicy Bypass -Command "& {{$env:AHQDB_INSTALL_DIR=`"{}`"; $env:EXECUTION_MODE=`"UserMode`"; (cat `"{}`" | iex)}}""#, parsed_path, script_path);      

      Command::new("powershell.exe")
        .arg(command)
        .creation_flags(0x08000000)
        .current_dir(ahqdb_dir)
        .spawn()?;
    }
  }

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