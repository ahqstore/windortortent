use std::{borrow::Cow, ptr::null_mut};

use std::fs;

use crate::{
  utils::{common_desktop, common_start_menu, user_desktop, user_start_menu},
  zip::ZipShortcut,
};

use windows::Win32::{
  System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance, IPersistFile},
  UI::Shell::{IShellLinkW, ShellLink},
};
use windows_core::Interface;
use windows_core::{HSTRING, PCWSTR, Result};

#[derive(Debug, Clone)]
pub enum Type {
  CurrentUser,
  AllUsers,
}

#[derive(Debug)]
pub enum ShortcutCreationInfo {
  AllOk,
  DesktopShortcutNotCreated,
}

pub fn unlink<'a>(link: &ZipShortcut<'a>, app_id: &str, folder: Type) -> Result<()> {
  let (desktop, mut start) = match folder {
    Type::AllUsers => (Cow::Borrowed(common_desktop()), common_start_menu()),
    Type::CurrentUser => (Cow::Owned(user_desktop()?), user_start_menu()?),
  };

  if let Some(dir) = link.start_menu_dir {
    start.push(dir);
    start.push(app_id);
  }

  fs::remove_dir_all(start)?;

  let src = format!(r"{desktop}\{} ({}).lnk", link.name, app_id);

  _ = fs::remove_file(src);

  Ok(())
}

pub fn link<'a, T: AsRef<str>>(
  link: &ZipShortcut<'a>,
  cwd: T,
  app_id: &str,
  folder: Type,
) -> Result<ShortcutCreationInfo> {
  unsafe {
    let data: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;

    let exe = format!("{}/{}", cwd.as_ref(), link.exe);
    let exe = fs::canonicalize(exe)?;
    let exe = exe.into_os_string();
    let pszfile: String = exe.into_string().unwrap_or("??".to_string());
    let pszfile = pszfile.replace(r"\\?\", "");

    let string = HSTRING::from(&pszfile);
    data.SetPath(PCWSTR::from_raw(string.as_ptr()))?;

    if let Some(args) = link.args {
      let string = HSTRING::from(args);
      data.SetArguments(PCWSTR::from_raw(string.as_ptr()))?;
    }

    if let Some((path, iicon)) = link.icon {
      let icon = HSTRING::from(path);

      data.SetIconLocation(PCWSTR::from_raw(icon.as_ptr()), iicon)?;
    } else {
      data.SetIconLocation(PCWSTR::from_raw(string.as_ptr()), 0)?;
    }

    if let Some(x) = link.description {
      let string = HSTRING::from(x);

      data.SetDescription(PCWSTR::from_raw(string.as_ptr()))?;
    }

    let mut file = null_mut();

    data
      .query(&IPersistFile::IID, &mut file as *mut _ as _)
      .ok()?;

    let file: IPersistFile = IPersistFile::from_raw(file);

    let mut start = match folder {
      Type::CurrentUser => user_start_menu()?,
      Type::AllUsers => common_start_menu(),
    };

    if let Some(dir) = link.start_menu_dir {
      start.push(dir);
    }

    start.push(app_id);
    _ = fs::create_dir_all(&start);

    start.push(format!("{}.lnk", link.name));

    let start = start.to_string_lossy();

    let fl = HSTRING::from(&start as &str);
    let string = PCWSTR::from_raw(fl.as_ptr());

    file.Save(string, true)?;

    let mut status = ShortcutCreationInfo::AllOk;

    if link.desktop {
      let fl = match folder {
        Type::CurrentUser => format!(r"{}\{} ({}).lnk", user_desktop()?, link.name, app_id),
        Type::AllUsers => format!(r"{}\{} ({}).lnk", common_desktop(), link.name, app_id),
      };
      let hstr = HSTRING::from(fl.as_str());
      let pt = hstr.as_ptr();
      let string = PCWSTR::from_raw(pt);

      if let Err(_) = file.Save(string, true) {
        status = ShortcutCreationInfo::DesktopShortcutNotCreated;
      }

      drop(hstr);
      drop(fl);
    }

    drop(pszfile);
    drop(start);
    drop(fl);

    Ok(status)
  }
}
