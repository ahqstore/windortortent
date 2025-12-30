//! This crate defines the newer async-friendly version of AHQDB
//! This is not the same as the older AHQ DB
//!
//! This ahqdb outputs a list of instructions to perform
//! as an outputs
//!
//! after extracting the ahqdb
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use rhai::Engine;

pub enum ActionInstruction {
  /// Download the data and copy it to the location (relative)
  DownloadCopy {
    asset: String,
    path: String,
  },
  DownloadUnzip {
    asset: String,
    dir: String,
  },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AHQDBManifest {
  windows: AHQDBWindowsManifest,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AHQDBWindowsManifest {
  aumid: Option<String>,
  links: Vec<AHQDBWindowsLink>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AHQDBWindowsLink {
  name: String,
  exe: String,
  description: Option<String>,
  args: Option<String>,
  // Default (exe, 0)
  icon: Option<(String, i32)>,
}

pub struct AHQDB<'a> {
  app_id: Cow<'a, str>,
}

impl<'a> AHQDB<'a> {}
