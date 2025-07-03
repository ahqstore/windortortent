use std::fs;
use std::mem::transmute;
use std::path::Path;
use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;
use zip::ZipArchive;

use crate::ahqdb::ps1::install::{run_install_ps1, run_uninstall_ps1};
use crate::ahqdb::ps1::run_is_installed_ps1;
use crate::utils::is_admin;
use crate::zip::link::{link, unlink, ShortcutCreationInfo, Type};
use crate::zip::ZipShortcut;

mod ps1;

pub struct AHQDBApplication<'a> {
  app_id: &'a str,
  version: &'a str,
  file: ZipArchive<File>,
  shortcut: BasicShortcutInfo<'a>,
  shortcut_info: Option<ADBShortcut>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LinkToml {
  pub link: ADBShortcut,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ADBShortcut {
  pub name: String,
  pub exe: String,
  pub ignore: Option<bool>,
  pub args: Option<String>,
  pub description: Option<String>,
  pub icon: Option<(String, i32)>,
}

pub struct BasicShortcutInfo<'a> {
  pub desktop: bool,
  pub start_menu_folder: Option<&'a str>,
}

#[derive(Debug)]
pub enum AHQDBError {
  ZipError(zip::result::ZipError),
  TokioIO(tokio::io::Error),
  TokioJoinError(tokio::task::JoinError),
  StdIO(std::io::Error),
  Windows(windows::core::Error),
  Toml(toml::de::Error),
  NotElevated,
  InvalidAHQDBFile,
  InvalidOutCode(i32)
}

impl From<tokio::task::JoinError> for AHQDBError {
  fn from(value: tokio::task::JoinError) -> Self {
    Self::TokioJoinError(value)
  }
}

impl From<zip::result::ZipError> for AHQDBError {
  fn from(value: zip::result::ZipError) -> Self {
    Self::ZipError(value)
  }
}

impl From<tokio::io::Error> for AHQDBError {
  fn from(value: tokio::io::Error) -> Self {
    Self::TokioIO(value)
  }
}

impl From<windows::core::Error> for AHQDBError {
  fn from(value: windows::core::Error) -> Self {
    Self::Windows(value)
  }
}

impl From<toml::de::Error> for AHQDBError {
  fn from(value: toml::de::Error) -> Self {
    Self::Toml(value)
  }
}

impl<'a> AHQDBApplication<'a> {
  pub fn new<T: AsRef<str>>(
    app_id: &'a str,
    path: T,
    version: &'a str,
    shortcut: BasicShortcutInfo<'a>,
  ) -> Result<Self, AHQDBError> {
    let path = path.as_ref();
    let file = File::open(path)?;

    let archive = ZipArchive::new(file)?;

    let mut out = Self {
      app_id,
      file: archive,
      shortcut,
      shortcut_info: None,
      version,
    };

    // Populate shortcut_info
    out.shortcut_info = Some(out.verify()?);

    Ok(out)
  }

  fn verify(&mut self) -> Result<ADBShortcut, AHQDBError> {
    let install = self.file.by_name("install.ps1")?.is_file();
    let uninstall = self.file.by_name("uninstall.ps1")?.is_file();
    let update = self.file.by_name("update.ps1")?.is_file();
    let is_installed = self.file.by_name("isInstalled.ps1")?.is_file();
    let _build = self.file.by_name(".build")?.is_file();

    let dist = self.file.by_name("dist/")?.is_dir();

    let mut link_data = self.file.by_name("link.toml")?;
    let link = link_data.is_file();

    if install && uninstall && update && is_installed && link && dist {
      let mut string = format!("");
      link_data.read_to_string(&mut string)?;

      let info: LinkToml = toml::from_str(&string)?;

      return Ok(info.link);
    }
    Err(AHQDBError::InvalidAHQDBFile)
  }

  /// SAFETY:
  /// This function uses `unsafe` and strict caller discipline to avoid undefined behavior
  /// The function must be awaited and resolved before self goes out of scope
  pub async fn async_is_installed(
    self,
    dir: String,
    ty: Type
  ) -> Result<(bool, Self), AHQDBError> {
    let data: AHQDBApplication<'static> = unsafe { transmute(self) };

    spawn_blocking(move || {
      let mut data = data;

      let out = data.is_installed(dir, ty)?;
      let result = (out, data);

      Ok(result)
    }).await?
  }

  pub fn is_installed<T: AsRef<str>>(
    &mut self,
    dir: T,
    ty: Type
  ) -> Result<bool, AHQDBError> {
    let dir = dir.as_ref();

    // This is directory where the files are extracted
    let script = format!(r"{dir}\ahqdb\isInstalled.ps1");

    // AHQ Database Dir
    let dist = format!(r"{dir}\dist_{}", self.version);

    run_is_installed_ps1(&script, &dist, &ty)
  }

  /// ## SAFETY:
  /// This function uses `unsafe` and strict caller discipline to avoid undefined behavior
  /// The function must be awaited and resolved before self goes out of scope
  pub async fn async_uninstall(
    self,
    dir: String,
    ty: Type
  ) -> Result<Self, AHQDBError> {
    let data: AHQDBApplication<'static> = unsafe { transmute(self) };

    spawn_blocking(move || {
      let mut data = data;

      data.uninstall(dir, ty)?;

      Ok(data)
    }).await?
  }

  pub fn uninstall<T: AsRef<str>>(
    &mut self,
    dir: T,
    ty: Type,
  ) -> Result<(), AHQDBError> {
    if let Type::AllUsers = ty {
      if !is_admin().unwrap_or(false) {
        return Err(AHQDBError::NotElevated);
      }
    }

    let dir = dir.as_ref();

    // This is directory where the files are extracted
    let dist = format!(r"{dir}\dist_{}", self.version);

    // Powershell Step
    // Runs uninstall.ps1
    run_uninstall_ps1("../ahqdb/uninstall.ps1", &dist, &ty)?;

    // Safety
    // it'll never ever panic, guaranteed by the [`Self::new`] function
    let ADBShortcut {
      args,
      description,
      exe,
      icon,
      ignore,
      name
    } = self.shortcut_info.as_ref().unwrap();

    if !ignore.unwrap_or(false) {
      let shortcut = ZipShortcut {
        args: args.as_deref(),
        description: description.as_deref(),
        exe: exe.as_str(),
        desktop: self.shortcut.desktop,
        start_menu_dir: self.shortcut.start_menu_folder.as_deref(),
        icon: icon.as_ref().map(|(string, num)| 
          (string.as_ref(), *num)
        ),
        name: name.as_str()
      };

      unlink(&shortcut, self.app_id, ty)?;
    }

    // Remove old ahqdb_dist completely
    fs::remove_dir_all(dir)?;

    Ok(())
  }

  /// ## SAFETY:
  /// This function uses `unsafe` and strict caller discipline to avoid undefined behavior
  /// The function must be awaited and resolved before self goes out of scope
  pub async fn async_install(
    self,
    dir: String,
    ty: Type
  ) -> Result<(ShortcutCreationInfo, Self), AHQDBError> {
    let data: AHQDBApplication<'static> = unsafe { transmute(self) };

    spawn_blocking(move || {
      let mut data = data;

      let info = data.install(dir, ty)?;

      Ok((info, data))
    }).await?
  }

  pub fn install<T: AsRef<str>>(
    &mut self,
    dir: T,
    ty: Type,
  ) -> Result<ShortcutCreationInfo, AHQDBError> {
    if let Type::AllUsers = ty {
      if !is_admin().unwrap_or(false) {
        return Err(AHQDBError::NotElevated);
      }
    }

    let dir = dir.as_ref();

    let dist_final = format!(r"{dir}\dist_{}", self.version);

    _ = fs::remove_dir_all(dir);
    fs::create_dir_all(&dist_final)?;

    self.file.extract(format!(r"{dir}\ahqdb"))?;

    let ahqdb_dist = format!(r"{dir}\ahqdb\dist");
    copy_dir_all(&ahqdb_dist, &dist_final)?;
    
    // Remove just deployed ahqdb_dist
    fs::remove_dir_all(ahqdb_dist)?;

    // Powershell Step
    run_install_ps1("../ahqdb/install.ps1", &dist_final, &ty)?;

    // Safety
    // it'll never ever panic, guaranteed by the [`Self::new`] function
    let ADBShortcut {
      args,
      description,
      exe,
      icon,
      ignore,
      name
    } = self.shortcut_info.as_ref().unwrap();

    let mut out = ShortcutCreationInfo::AllOk;

    if !ignore.unwrap_or(false) {
      let shortcut = ZipShortcut {
        args: args.as_deref(),
        description: description.as_deref(),
        exe: exe.as_str(),
        desktop: self.shortcut.desktop,
        start_menu_dir: self.shortcut.start_menu_folder.as_deref(),
        icon: icon.as_ref().map(|(string, num)| 
          (string.as_ref(), *num)
        ),
        name: name.as_str()
      };

      out = link(&shortcut, &dist_final, self.app_id, ty)?;
    }

    // We'll keep ahqdb folder for the powershell 
    // Hence its not removed
    Ok(out)
  }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
  fs::create_dir_all(&dst)?;
  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let ty = entry.file_type()?;
    if ty.is_dir() {
      copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
    } else {
      fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
    }
  }
  Ok(())
}
