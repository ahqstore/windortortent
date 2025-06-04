use windows::Win32::{
  Foundation::HANDLE, 
  Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY}, 
  System::Threading::{GetCurrentProcessId, OpenProcess, OpenProcessToken, PROCESS_QUERY_LIMITED_INFORMATION}, UI::Shell::GetUserProfileDirectoryW
};
use windows_core::{Result, PCWSTR, PWSTR};

pub fn is_admin() -> Result<bool> {
  unsafe {
    let process = OpenProcess(
      PROCESS_QUERY_LIMITED_INFORMATION,
      false, 
      GetCurrentProcessId()
    )?;

    let mut tokenhandle = HANDLE::default();

    _ = OpenProcessToken(
      process,
      TOKEN_QUERY,
      &mut tokenhandle
    )?;

    let mut info = TOKEN_ELEVATION::default();
    let mut len = 0;

    _ = GetTokenInformation(
      tokenhandle, 
      TokenElevation, 
      Some(&mut info as *mut _ as _), 
      size_of_val(&info) as u32, 
      &mut len
    )?;

    Ok(info.TokenIsElevated != 0)
  }
}

pub fn user_profile_dir() -> Result<String> {
  unsafe {
    let processhandle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, GetCurrentProcessId())?;

    let mut htoken = HANDLE::default();

    OpenProcessToken(
      processhandle, 
      TOKEN_QUERY, 
      &mut htoken
    )?;

    let mut size = 0u32;

    // This errors out & fills the size
    _ = GetUserProfileDirectoryW(
      htoken, 
      None, 
      &mut size
    );

    let mut data = vec![0u16; size as usize];
    let dir = PWSTR::from_raw(data.as_mut_ptr());

    GetUserProfileDirectoryW(
      htoken, 
      Some(dir), 
      &mut size
    )?;

    Ok(dir.to_string()?)
  }
}

pub fn user_desktop() -> Result<String> {
  Ok(format!("{}\\Desktop", user_profile_dir()?))
}

pub const fn common_desktop() -> &'static str {
  r"C:\Users\Default\Desktop"
}