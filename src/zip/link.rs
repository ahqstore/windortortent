use std::ptr::null_mut;

use tokio::fs;

use crate::zip::ZipShortcut;

use windows::Win32::{System::Com::{CoCreateInstance, IPersistFile, CLSCTX_INPROC_SERVER}, UI::Shell::{IShellLinkW, ShellLink}};
use windows_core::{Result, HSTRING, PCWSTR};
use windows_core::Interface;

pub enum Type {
  CurrentUser,
  AllUsers
}

pub async fn link<'a>(
  link: &ZipShortcut<'a>, 
  cwd: String,
  folder: Type
) -> Result<()> {
  unsafe { 
    let data: IShellLinkW = CoCreateInstance(
      &ShellLink,
      None,
      CLSCTX_INPROC_SERVER
    )?;

    let exe = format!("{cwd}/{}", link.exe);
    let exe = fs::canonicalize(exe).await?;
    let exe = exe.into_os_string();
    let pszfile: String = exe.into_string().unwrap_or("??".to_string());
    let pszfile = pszfile.replace(r"\\?\", "");

    let string = HSTRING::from(&pszfile);
    data.SetPath(
      PCWSTR::from_raw(string.as_ptr())
    )?;

    if let Some(args) = link.args {
      let string = HSTRING::from(args);
      data.SetArguments(
        PCWSTR::from_raw(string.as_ptr())
      )?;
    }

    if let Some((path, iicon)) = link.icon {
      let icon = HSTRING::from(path);

      data.SetIconLocation(
        PCWSTR::from_raw(icon.as_ptr()), 
        iicon
      )?;
    } else {
      data.SetIconLocation(
        PCWSTR::from_raw(string.as_ptr()), 
        0
      )?;
    }

    if let Some(x) = link.description {
      let string = HSTRING::from(x);

      data.SetDescription(
        PCWSTR::from_raw(string.as_ptr())
      )?;
    }

    let mut file = null_mut();

    data.query(
      &IPersistFile::IID,
      &mut file as *mut _ as _
    ).ok()?;

    let file: IPersistFile = IPersistFile::from_raw(file);

    println!("{file:#?}");
    file.Save(
      windows::core::w!(r"E:\GitHub\windortortent\app.lnk"), true)?;

    Ok(())
  }
}