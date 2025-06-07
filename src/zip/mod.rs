pub mod link;

use std::fs::{self, File};
pub use zip::read::ZipArchive;
pub use zip::result::ZipError;
pub use zip::write::ZipWriter;

use crate::utils::is_admin;
use crate::zip::link::{ShortcutCreationInfo, Type};

#[derive(Debug)]
pub struct ZipInstaller<'a> {
  id: &'a str,
  file: ZipArchive<File>,
  shortcut: ZipShortcut<'a>,
}

#[derive(Debug)]
pub struct ZipShortcut<'a> {
  pub name: &'a str,
  pub exe: &'a str,
  pub args: Option<&'a str>,
  pub description: Option<&'a str>,
  pub icon: Option<(&'a str, i32)>,
  pub desktop: bool,
  pub start_menu_dir: Option<&'a str>,
}

#[derive(Debug)]
pub enum ZipInstallError {
  ZipError(zip::result::ZipError),
  TokioIO(tokio::io::Error),
  StdIO(std::io::Error),
  Windows(windows::core::Error),
  NotElevated,
}

impl From<zip::result::ZipError> for ZipInstallError {
  fn from(value: zip::result::ZipError) -> Self {
    Self::ZipError(value)
  }
}

impl From<tokio::io::Error> for ZipInstallError {
  fn from(value: tokio::io::Error) -> Self {
    Self::TokioIO(value)
  }
}

impl From<windows::core::Error> for ZipInstallError {
  fn from(value: windows::core::Error) -> Self {
    Self::Windows(value)
  }
}

impl<'a> ZipInstaller<'a> {
  pub fn new<T: AsRef<str>>(file: T, id: &'a str, data: ZipShortcut<'a>) -> Result<Self, ZipInstallError> {
    let file = file.as_ref();
    let file = File::open(file).map_err(ZipInstallError::StdIO)?;

    Ok(ZipInstaller {
      file: ZipArchive::new(file)?,
      id,
      shortcut: data,
    })
  }

  pub fn install<T: AsRef<str>>(
    &mut self,
    dir: T,
    ty: Type,
  ) -> Result<ShortcutCreationInfo, ZipInstallError> {
    is_admin()
      .map_err(ZipInstallError::Windows)
      .and_then(|is_admin| {
        if !is_admin {
          if let Type::AllUsers = ty {
            return Err(ZipInstallError::NotElevated);
          }
        }

        Ok(())
      })?;

    // Ensuring a cleaned directory
    _ = fs::remove_dir_all(dir.as_ref());
    _ = fs::create_dir_all(dir.as_ref());

    let res = self
      .file
      .extract(dir.as_ref())
      .map_err(ZipInstallError::ZipError);

    if res.is_err() {
      // Cleanup
      _ = fs::remove_dir_all(dir.as_ref());
    }

    res?;

    // Create Win32 Shortcut
    let status = link::link(&self.shortcut, dir.as_ref(), self.id, ty)?;

    Ok(status)
  }

  pub fn uninstall<T: AsRef<str>>(&mut self, dir: T, ty: Type) -> Result<(), ZipInstallError> {
    is_admin()
      .map_err(ZipInstallError::Windows)
      .and_then(|is_admin| {
        if !is_admin {
          if let Type::AllUsers = ty {
            return Err(ZipInstallError::NotElevated);
          }
        }

        Ok(())
      })?;

    // Ensuring a cleaned directory
    _ = fs::remove_dir_all(dir.as_ref());

    link::unlink(&self.shortcut, self.id, ty)?;

    Ok(())
  }
}
