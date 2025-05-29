use std::{fs::File, io::Read};
use tokio::task::spawn_blocking;
use windows_core::Result as Win32Result;

use zip::read::ZipArchive;

use serde::{Serialize, Deserialize};
use serde_xml_rs::from_str;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bundle {
  #[serde(rename = "Identity")]
  pub identity: Identity
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@Version")]
  pub version: String,
  #[serde(rename = "@Publisher")]
  pub publisher: String,
}

use super::UWPPackageManager;

#[non_exhaustive]
pub struct MsixBundle {
  pub path: String,
  pub identity: Identity
}

impl AsRef<str> for MsixBundle {
  fn as_ref(&self) -> &str {
    &self.path
  }
}

pub type MsixBundleResult<T> = Result<T, MsixBundleError>;

pub enum MsixBundleError {
  JoinError(tokio::task::JoinError),
  TokioIO(tokio::io::Error),
  ZipError(zip::result::ZipError),
  Serde(serde_xml_rs::Error),
}

impl From<serde_xml_rs::Error> for MsixBundleError {
  fn from(value: serde_xml_rs::Error) -> Self {
    Self::Serde(value)
  }
}

impl From<tokio::task::JoinError> for MsixBundleError {
  fn from(value: tokio::task::JoinError) -> Self {
    Self::JoinError(value)
  }
}

impl From<zip::result::ZipError> for MsixBundleError {
  fn from(value: zip::result::ZipError) -> Self {
    Self::ZipError(value)
  }
}

impl From<tokio::io::Error> for MsixBundleError {
  fn from(value: tokio::io::Error) -> Self {
    Self::TokioIO(value)
  }
}

impl MsixBundle {
  pub async fn load<T: Into<String>>(path: T) -> MsixBundleResult<Self> {
    let path: String = path.into();

    spawn_blocking(move || {
      let path = path;
      let file = File::open(&path)?;

      let mut archive = ZipArchive::new(file)?;

      let mut string = String::new();

      archive.by_name("AppxMetadata/AppxBundleManifest.xml")?.read_to_string(&mut string)?;

      let bundle: Bundle = from_str(&string)?;

      MsixBundleResult::Ok(
        MsixBundle {
          path,
          identity: bundle.identity
        }
      )
    }).await?
  }

  pub async fn install(&self, manager: &UWPPackageManager) -> Win32Result<()> {
    manager.install(&self).await
  }

  // pub async fn is_installed(&self, manager: &UWPPackageManager) -> Result<bool> {
  //   let info = manager.get_intalled_info_sync(&self, &"")?;
  // }

  pub async fn uninstall(&self, manager: &UWPPackageManager) -> Win32Result<()> {
    manager.remove(&self).await
  }
}