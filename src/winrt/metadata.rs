use std::{fs::File, io::Read, path::Path, sync::Arc};
use tokio::task::spawn_blocking;
use windows::ApplicationModel::PackageVersion;
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
#[derive(Debug)]
pub struct MsixBundle {
  pub path: String,
  pub identity: Identity,
  pub full_name: Option<String>,
  pub manager: Arc<UWPPackageManager>
}

impl AsRef<str> for MsixBundle {
  fn as_ref(&self) -> &str {
    &self.path
  }
}

pub type MsixBundleResult<T> = Result<T, MsixBundleError>;

#[derive(Debug)]
pub enum MsixBundleError {
  JoinError(tokio::task::JoinError),
  TokioIO(tokio::io::Error),
  ZipError(zip::result::ZipError),
  Serde(serde_xml_rs::Error),
  Win32(windows_core::Error)
}

impl From<windows_core::Error> for MsixBundleError {
  fn from(value: windows_core::Error) -> Self {
    MsixBundleError::Win32(value)
  }
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
  pub async fn load<T: AsRef<Path>>(path: T, manager: &Arc<UWPPackageManager>) -> MsixBundleResult<Self> {
    let path = tokio::fs::canonicalize(path).await?;
    let path = path.to_str().unwrap_or("");
    let path = path.get(4..).unwrap_or("");
    let path = path.to_string();

    let manager = manager.clone();

    spawn_blocking(move || {
      let path = path;
      let file = File::open(&path)?;

      let mut archive = ZipArchive::new(file)?;

      let mut string = String::new();

      archive.by_name("AppxMetadata/AppxBundleManifest.xml")?.read_to_string(&mut string)?;

      let bundle: Bundle = from_str(&string)?;

      let mut bundle = MsixBundle {
        path,
        identity: bundle.identity,
        full_name: None,
        manager
      };

      bundle.reload_install_status()?;

      MsixBundleResult::Ok(
        bundle
      )
    }).await?
  }

  pub fn reload_install_status(&mut self) -> MsixBundleResult<()> {
    let identity = &self.identity;
    let info = self.manager.get_intalled_info_sync(&identity.name, &identity.publisher)?;

    let pkg = info.into_iter().find(|x| {
      (|| {
        let author = x.Id()?.Publisher()?;
        let name = x.Id()?.Name()?;

        let PackageVersion {
          Build,
          Major,
          Minor,
          Revision
        } = x.Id()?.Version()?;

        let version = format!("{Major}.{Minor}.{Build}.{Revision}");

        Win32Result::Ok(
          &identity.name == &name &&
          &identity.publisher == &author &&
          &identity.version == &version
        )
      })().unwrap_or(false)
    });

    if let Some(pkg) = pkg {
      let name = pkg.Id()?.FullName()?;
      let name = name.to_string_lossy();

      self.full_name = Some(name);
    }

    Ok(())
  }

  pub async fn install(&self) -> Win32Result<()> {
    self.manager.install(&self).await
  }

  pub fn is_installed(&mut self) -> MsixBundleResult<bool> {
    self.reload_install_status()?;

    Ok(self.full_name.is_some())
  }

  /// Use [async_is_installed] instead
  /// 
  /// ***SAFETY***
  /// 
  /// Await this as soon as you call this function
  /// 
  /// THIS FUNCTION USES `UNSAFE` CASTING TO MARK THE VARIABLE AS &'static
  #[deprecated(
    since = "0.1.0",
    note = "This method will not be removed! Use `async_is_installed` instead, as it provides a safe, idiomatic alternative by transferring ownership. This function relies on `unsafe` and strict caller discipline to avoid undefined behavior."
  )]
  pub async unsafe fn async_unsafe_is_installed<'a>(&'a mut self) -> Result<bool, MsixBundleError> {
    let me: &'static mut MsixBundle = unsafe { &mut *(self as *mut _) as &'static mut _ };

    let is_installed = spawn_blocking(|| {
      me.is_installed()
    }).await??;

    Ok(
      is_installed
    )
  }

  #[allow(unused_mut)]
  pub async fn async_is_installed(mut self) -> Result<(Self, bool), MsixBundleError> {
    let mut me = self;

    let is_installed = spawn_blocking(move || {
      let result = me.is_installed()?;

      MsixBundleResult::Ok((me, result))
    }).await??;

    Ok(
      is_installed
    )
  }

  pub async fn uninstall(&self) -> Win32Result<()> {
    let full_name = self.full_name
      .as_ref()
      .map_or_else(|| "", |x| x.as_str());

    self.manager.remove(full_name).await
  }
}