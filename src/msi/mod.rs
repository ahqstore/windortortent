use std::fs;

use windows::{
  Win32::System::ApplicationInstallationAndServicing::{
    INSTALLSTATE_ABSENT, INSTALLSTATE_ADVERTISED, INSTALLSTATE_DEFAULT, INSTALLSTATE_UNKNOWN,
    MSIDBOPEN_READONLY, MSIHANDLE, MsiCloseHandle, MsiDatabaseOpenViewW, MsiInstallProductW,
    MsiOpenDatabaseW, MsiQueryProductStateW, MsiRecordGetStringW, MsiViewExecute, MsiViewFetch,
  },
  core::w,
};
use windows_core::{HRESULT, HSTRING, PCWSTR, PWSTR, Result};

/// This is an MSI Package with a `ProductCode`
pub struct MsiPackage(String, String);

#[derive(Debug)]
pub enum ProductState {
  NotInstalled = -1,
  AdvertisedButNotInstalled = 1,
  InstalledForDifferentUser = 2,
  Installed = 5,
  Unknown,
}

impl MsiPackage {
  pub unsafe fn new_from_product_code<E: Into<String>, T: Into<String>>(
    path: E,
    product_code: T,
  ) -> Result<MsiPackage> {
    Ok(MsiPackage(path.into(), product_code.into()))
  }

  pub fn product_state(&self) -> ProductState {
    unsafe {
      let data = PCWSTR::from_raw(HSTRING::from(self.1.as_ref() as &str).as_ptr());

      match MsiQueryProductStateW(data) {
        INSTALLSTATE_ABSENT => ProductState::InstalledForDifferentUser,
        INSTALLSTATE_ADVERTISED => ProductState::AdvertisedButNotInstalled,
        INSTALLSTATE_DEFAULT => ProductState::Installed,
        INSTALLSTATE_UNKNOWN => ProductState::NotInstalled,
        _ => ProductState::Unknown,
      }
    }
  }

  pub fn is_installed(&self) -> bool {
    match self.product_state() {
      ProductState::Installed => true,
      _ => false,
    }
  }

  pub fn install(&self) -> Result<()> {
    unsafe {
      let hstring = HSTRING::from(self.0.as_ref() as &str);

      let path = PCWSTR::from_raw(hstring.as_ptr());

      let res = MsiInstallProductW(path, w!("ACTION=INSTALL UILevel=3"));

      HRESULT::from_win32(res).ok()?;

      Ok(())
    }
  }

  pub fn uninstall(&self) -> Result<()> {
    unsafe {
      let hstring = HSTRING::from(self.0.as_ref() as &str);

      let path = PCWSTR::from_raw(hstring.as_ptr());

      let res = MsiInstallProductW(path, w!("REMOVE=ALL UILevel=3"));

      HRESULT::from_win32(res).ok()?;

      Ok(())
    }
  }

  /// Creates a new MsiPackage by opening the specified MSI file,
  /// extracting its ProductCode, and then closing the database.
  ///
  /// # Arguments
  /// * `name` - The relativ/absolute path to the MSI file.
  ///
  /// # Returns
  /// A `Result` containing an `MsiPackage` with the ProductCode on success,
  /// or a `windows_core::Error` on failure.
  pub fn new<T: AsRef<str>>(name: T) -> Result<MsiPackage> {
    let mut hwnd = MSIHANDLE::default();

    Ok(unsafe {
      let name = name.as_ref();

      let fetch = fs::canonicalize(name)?;
      let name = fetch.to_string_lossy();
      let name = name.as_ref().get(4..).unwrap_or(r"\\?\");

      let name_return = name.to_string();

      let name = HSTRING::from(name);
      let name = PCWSTR::from_raw(name.as_ptr());

      let err = MsiOpenDatabaseW(name, MSIDBOPEN_READONLY, &mut hwnd);

      HRESULT::from_win32(err).ok()?;

      let mut view = MSIHANDLE::default();

      HRESULT::from_win32(MsiDatabaseOpenViewW(
        hwnd,
        w!("SELECT `Value` FROM `Property` WHERE `Property` = 'ProductCode'"),
        &mut view,
      ))
      .ok()?;

      HRESULT::from_win32(MsiViewExecute(view, MSIHANDLE::default())).ok()?;

      let mut record = MSIHANDLE::default();
      let phrecord = &mut record as *mut MSIHANDLE;

      HRESULT::from_win32(MsiViewFetch(view, phrecord)).ok()?;

      let mut string = [0u16; 39];

      HRESULT::from_win32(MsiRecordGetStringW(
        record,
        1,
        Some(PWSTR::from_raw(&mut string as *mut _ as _)),
        Some(&mut 39),
      ))
      .ok()?;

      let data = HSTRING::from_wide(&string).to_string_lossy();

      HRESULT::from_win32(MsiCloseHandle(hwnd)).ok()?;

      MsiPackage(name_return, data)
    })
  }
}
