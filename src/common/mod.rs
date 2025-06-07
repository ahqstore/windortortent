use std::i32;
use std::ptr::null;
use std::time::Duration;

use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::System::Threading::GetExitCodeProcess;
use windows::Win32::UI::Shell::{ShellExecuteExW, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW};
use windows::core::w;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use windows_core::{Error, Result, HRESULT, HSTRING, PCWSTR};

pub fn run_as_admin(path: &str) -> Result<()> {
  unsafe {
    let file = HSTRING::from(path);
    let lpfile = file.as_ptr();
    let lpfile = PCWSTR::from_raw(lpfile);

    let mut pexecinfo = SHELLEXECUTEINFOW {
      lpVerb: w!("runas"),
      lpFile: lpfile,
      nShow: SW_SHOWNORMAL.0,
      fMask: SEE_MASK_NOCLOSEPROCESS,
      hwnd: HWND::default(),
      hInstApp: HINSTANCE::default(),
      lpDirectory: PCWSTR(null()),
      lpParameters: PCWSTR(null()),
      ..Default::default()
    };

    pexecinfo.cbSize = size_of::<SHELLEXECUTEINFOW>() as u32;

    ShellExecuteExW(&mut pexecinfo).unwrap();

    let hprocess = pexecinfo.hProcess;

    let mut exit_code = 0u32;
    
    loop {
      GetExitCodeProcess(hprocess, &mut exit_code)?;

      if exit_code == 259 {
        std::thread::sleep(Duration::from_millis(10));
        continue;
      }

      if exit_code == 0 {
        return Ok(());
      } else {
        return Err(Error::new(HRESULT::from_nt(i32::MAX), "RIP"));
      }
    }
  }
}