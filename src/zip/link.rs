use std::ptr::null_mut;

use std::fs;

use crate::{
  utils::{common_desktop, user_desktop},
  zip::ZipShortcut,
};

use windows::Win32::{
  System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance, IPersistFile},
  UI::Shell::{IShellLinkW, ShellLink},
};
use windows_core::Interface;
use windows_core::{HSTRING, PCWSTR, Result};

pub enum Type {
  CurrentUser,
  AllUsers,
}

pub fn link<'a, T: AsRef<str>>(link: &ZipShortcut<'a>, cwd: T, folder: Type) -> Result<()> {
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

    if link.desktop {
      let fl = match folder {
        Type::CurrentUser => format!(r"{}\{}.lnk", user_desktop()?, link.name),
        Type::AllUsers => format!(r"{}\{}.lnk", common_desktop(), link.name),
      };
      println!("{}", fl);
      let hstr = HSTRING::from(fl.as_str());
      let string = PCWSTR::from_raw(hstr.as_ptr());

      file.Save(string, true)?;
    }

    Ok(())
  }
}
