pub mod link;

pub use zip::read::ZipArchive;
pub use zip::result::ZipError;
pub use zip::write::ZipWriter;

pub struct ZipInstaller<'a> {
  shortcut: ZipShortcut<'a>
}

pub struct ZipShortcut<'a> {
  pub name: &'a str,
  pub exe: &'a str,
  pub args: Option<&'a str>,
  pub description: Option<&'a str>,
  pub icon: Option<(&'a str, i32)>
}

pub enum ZipInstallError {
  ZipError(zip::result::ZipError),
  TokioIO(tokio::io::Error),
  NotElevated
}

