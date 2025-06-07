use std::ffi::OsStr;

use tokio::{io::Error, process::Command};

#[derive(Debug, Clone, Copy)]
pub enum SuccessStatus {
  Stat(bool),
  Unknown,
}

pub async fn install<T: AsRef<str>, E: IntoIterator<Item = S>, S: AsRef<OsStr>>(
  path: T,
  args: E,
  wait: bool,
) -> Result<SuccessStatus, Error> {
  let mut child = Command::new(path.as_ref()).args(args).spawn()?;

  if wait {
    return Ok(SuccessStatus::Stat(child.wait().await?.success()));
  }

  Ok(SuccessStatus::Unknown)
}