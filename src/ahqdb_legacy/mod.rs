use std::fs;
use std::mem::transmute;
use std::path::Path;
use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};
use serde_xml_rs::to_string;
use tokio::task::spawn_blocking;
use toml::from_str;
use zip::ZipArchive;

use crate::ahqdb_legacy::ps1::install::{run_install_ps1, run_uninstall_ps1};
use crate::ahqdb_legacy::ps1::run_is_installed_ps1;
use crate::ahqdb_legacy::ps1::update::run_update_ps1;
use crate::utils::is_admin;
use crate::zip::ZipShortcut;
use crate::zip::link::{ShortcutCreationInfo, Type, link, unlink};

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
  InvalidOutCode(i32),
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

    let mut archive = ZipArchive::new(file)?;

    let shortcut_info = Self::verify(&mut archive)?;

    let out = Self {
      app_id,
      file: archive,
      shortcut,
      shortcut_info: Some(shortcut_info),
      version,
    };

    Ok(out)
  }

  fn verify(file: &mut ZipArchive<File>) -> Result<ADBShortcut, AHQDBError> {
    let install = file.by_name("install.ps1")?.is_file();
    let uninstall = file.by_name("uninstall.ps1")?.is_file();
    let update = file.by_name("update.ps1")?.is_file();
    let is_installed = file.by_name("isInstalled.ps1")?.is_file();
    let _build = file.by_name(".build")?.is_file();

    let dist = file.by_name("dist/")?.is_dir();

    let mut link_data = file.by_name("link.toml")?;
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
  pub async fn async_is_installed(self, dir: String, ty: Type) -> Result<(bool, Self), AHQDBError> {
    let data: AHQDBApplication<'static> = unsafe { transmute(self) };

    spawn_blocking(move || {
      let mut data = data;

      let out = data.is_installed(dir, ty)?;
      let result = (out, data);

      Ok(result)
    })
    .await?
  }

  pub fn is_installed<T: AsRef<str>>(&mut self, dir: T, ty: Type) -> Result<bool, AHQDBError> {
    let dir = dir.as_ref();

    // This is directory where the files are extracted
    let script = format!(r"{dir}\ahqdb\isInstalled.ps1");

    // AHQ Database Dir
    let dist = format!(r"{dir}\dist_{}", self.version);

    run_is_installed_ps1(&script, &dist, &ty)
  }

  fn get_type<T: AsRef<str>>(&self, dir: T) -> Result<Type, AHQDBError> {
    let dir = dir.as_ref();

    let r#type = fs::read_to_string(format!(r"{dir}\ahqdb\type"))?;
    match r#type.as_str() {
      "all" => Ok(Type::AllUsers),
      "current" => Ok(Type::CurrentUser),
      _ => Err(AHQDBError::InvalidAHQDBFile),
    }
  }

  /// ## SAFETY:
  /// This function uses `unsafe` and strict caller discipline to avoid undefined behavior
  /// The function must be awaited and resolved before self goes out of scope
  pub async fn async_uninstall(self, dir: String) -> Result<Self, AHQDBError> {
    let data: AHQDBApplication<'static> = unsafe { transmute(self) };

    spawn_blocking(move || {
      let mut data = data;

      data.uninstall(dir)?;

      Ok(data)
    })
    .await?
  }

  pub fn uninstall<T: AsRef<str>>(&mut self, dir: T) -> Result<(), AHQDBError> {
    let ty = self.get_type(&dir)?;

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
      name,
    } = self.shortcut_info.as_ref().unwrap();

    if !ignore.unwrap_or(false) {
      let shortcut = ZipShortcut {
        args: args.as_deref(),
        description: description.as_deref(),
        exe: exe.as_str(),
        desktop: self.shortcut.desktop,
        start_menu_dir: self.shortcut.start_menu_folder.as_deref(),
        icon: icon.as_ref().map(|(string, num)| (string.as_ref(), *num)),
        name: name.as_str(),
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
  pub async fn async_update(self, dir: String) -> Result<(), AHQDBError> {
    let mut me: AHQDBApplication<'static> = unsafe { transmute(self) };

    spawn_blocking(move || me.update(dir)).await?
  }

  pub fn update<T: AsRef<str>>(&mut self, dir: T) -> Result<(), AHQDBError> {
    let ty = self.get_type(&dir)?;

    let dir = dir.as_ref();

    let old_ver = fs::read_to_string(format!(r"{dir}\ahqdb\.version"))?;

    let dist_old = format!(r"{dir}\dist_{old_ver}");

    let old_link: ADBShortcut = from_str(&fs::read_to_string(format!(r"{dir}\ahqdb\.link"))?)?;
    let old_build = fs::read_to_string(format!(r"{dir}\ahqdb\.build"))?;

    // Safety
    // it'll never ever panic, guaranteed by the [`Self::new`] function
    let ADBShortcut {
      args,
      description,
      exe,
      icon,
      ignore,
      name,
    } = &old_link;

    // Remove the old shortcut
    if !ignore.unwrap_or(false) {
      let shortcut = ZipShortcut {
        args: args.as_deref(),
        description: description.as_deref(),
        exe: exe.as_str(),
        desktop: self.shortcut.desktop,
        start_menu_dir: self.shortcut.start_menu_folder.as_deref(),
        icon: icon.as_ref().map(|(string, num)| (string.as_ref(), *num)),
        name: name.as_str(),
      };

      unlink(&shortcut, self.app_id, ty.clone())?;
    }

    let ahqdb_new = format!(r"{dir}\ahqdb_new");

    fs::create_dir_all(&ahqdb_new)?;

    self.file.extract(&ahqdb_new)?;

    let ahqdb_dist = format!(r"{dir}\ahqdb_new\dist");

    let dist_final = format!(r"{dir}\dist_{}", self.version);
    fs::create_dir_all(&dist_final)?;

    copy_dir_all(&ahqdb_dist, &dist_final)?;

    run_update_ps1(
      "../ahqdb_new/update.ps1",
      &dist_old,
      &dist_final,
      &old_build,
      &ty,
    )?;

    fs::remove_dir_all(dist_old)?;
    fs::remove_dir_all(format!(r"{dir}\ahqdb"))?;
    fs::rename(ahqdb_new, format!(r"{dir}\ahqdb"))?;

    // Add Type
    fs::write(
      format!(r"{dir}\ahqdb\type"),
      match ty {
        Type::AllUsers => "all",
        Type::CurrentUser => "current",
      },
    )?;

    // Add Version
    fs::write(format!(r"{dir}\ahqdb\.version"), self.version)?;

    // Guaranteed that to_string won't error out
    fs::write(
      format!(r"{dir}\ahqdb\.link"),
      to_string(self.shortcut_info.as_ref().unwrap()).map_err(|_| AHQDBError::InvalidAHQDBFile)?,
    )?;

    // Create the new shortcut
    //
    // Safety
    // it'll never ever panic, guaranteed by the [`Self::new`] function
    let ADBShortcut {
      args,
      description,
      exe,
      icon,
      ignore,
      name,
    } = self.shortcut_info.as_ref().unwrap();

    if !ignore.unwrap_or(false) {
      let shortcut = ZipShortcut {
        args: args.as_deref(),
        description: description.as_deref(),
        exe: exe.as_str(),
        desktop: self.shortcut.desktop,
        start_menu_dir: self.shortcut.start_menu_folder.as_deref(),
        icon: icon.as_ref().map(|(string, num)| (string.as_ref(), *num)),
        name: name.as_str(),
      };

      link(&shortcut, &dist_final, self.app_id, ty)?;
    }

    todo!()
  }

  /// ## SAFETY:
  /// This function uses `unsafe` and strict caller discipline to avoid undefined behavior
  /// The function must be awaited and resolved before self goes out of scope
  pub async fn async_install(
    self,
    dir: String,
    ty: Type,
  ) -> Result<(ShortcutCreationInfo, Self), AHQDBError> {
    let data: AHQDBApplication<'static> = unsafe { transmute(self) };

    spawn_blocking(move || {
      let mut data = data;

      let info = data.install(dir, ty)?;

      Ok((info, data))
    })
    .await?
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

    // Add Type
    fs::write(
      format!(r"{dir}\ahqdb\type"),
      match ty {
        Type::AllUsers => "all",
        Type::CurrentUser => "current",
      },
    )?;

    // Add Version
    fs::write(format!(r"{dir}\ahqdb\.version"), self.version)?;

    // Guaranteed that to_string won't error out
    fs::write(
      format!(r"{dir}\ahqdb\.link"),
      to_string(self.shortcut_info.as_ref().unwrap()).map_err(|_| AHQDBError::InvalidAHQDBFile)?,
    )?;

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
      name,
    } = self.shortcut_info.as_ref().unwrap();

    let mut out = ShortcutCreationInfo::AllOk;

    if !ignore.unwrap_or(false) {
      let shortcut = ZipShortcut {
        args: args.as_deref(),
        description: description.as_deref(),
        exe: exe.as_str(),
        desktop: self.shortcut.desktop,
        start_menu_dir: self.shortcut.start_menu_folder.as_deref(),
        icon: icon.as_ref().map(|(string, num)| (string.as_ref(), *num)),
        name: name.as_str(),
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
