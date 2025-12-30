use std::process::Stdio;
use tokio::process::Command;

use super::DEFENDER_CMD;

pub type Malicious = bool;

pub async fn scan(path: &str) -> Option<Malicious> {
  let out = Command::new(DEFENDER_CMD)
    .args(["-Scan", "-ScanType", "3", "-File"])
    .arg(path)
    .stdout(Stdio::piped())
    .spawn()
    .ok()?
    .wait_with_output()
    .await
    .ok()?
    .stdout;

  let okay = format!("Scanning {} found no threats", &path);

  let out = String::from_utf8_lossy(&out);

  if out.contains(&okay) {
    return Some(false);
  }

  Some(true)
}
