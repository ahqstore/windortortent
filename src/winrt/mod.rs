use std::sync::Arc;

use windows::{
  core::{Result, HSTRING, PWSTR}, ApplicationModel::Package, Foundation::Uri, Management::Deployment::{DeploymentOptions, PackageManager}, Win32::{
    Foundation::HANDLE,
    Security::{
      Authorization::ConvertSidToStringSidW, GetTokenInformation, TokenUser, TOKEN_QUERY, TOKEN_USER
    },
    System::Threading::{GetCurrentProcess, OpenProcessToken},
  }
};
pub mod metadata;

#[derive(Debug)]
pub struct MSIXPackageManager(PackageManager);

pub fn get_user_sid_string() -> Result<HSTRING> {
  unsafe {
    let mut token_handle: HANDLE = HANDLE::default();

    OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle)?;

    let mut len = 300;

    let mut info = vec![0u8; len as usize];

    GetTokenInformation(
      token_handle,
      TokenUser,
      Some(info.as_mut_ptr() as _),
      len,
      &mut len,
    )?;

    let mut sid = PWSTR::default();

    let info = &mut info[0usize..(len as usize)];

    let info = info.as_mut_ptr() as *mut TOKEN_USER;

    let val = (&mut *info).User.Sid;

    ConvertSidToStringSidW(val, &mut sid)?;

    Ok(sid.to_hstring())
  }
}

impl MSIXPackageManager {
  pub fn new() -> Result<Arc<Self>> {
    Ok(Arc::new(Self(PackageManager::new()?)))
  }

  pub async fn install<T: AsRef<str>>(&self, path: T) -> Result<()> {
    let path = path.as_ref();

    let path = HSTRING::from(path);

    let uri = Uri::CreateUri(&path)?;

    let prog = self
      .0
      .AddPackageAsync(&uri, None, DeploymentOptions::InstallAllResources)?;

    let result = prog.await?;

    result.ExtendedErrorCode()?.ok()
  }

  pub async fn remove<T: AsRef<str>>(&self, full_name: T) -> Result<()> {
    let full_name = full_name.as_ref();
    let full_name = HSTRING::from(full_name);
    let result = self.0.RemovePackageAsync(&full_name)?.await?;

    result.ExtendedErrorCode()?.ok()
  }

  pub fn get_intalled_info_sync<T: AsRef<str>, E: AsRef<str>>(&self, app_name: T, publisher: E) -> Result<Vec<Package>> {
    let pkg = self.0.FindPackagesByUserSecurityIdNamePublisher(
      &get_user_sid_string()?,
      &HSTRING::from(app_name.as_ref()),
      &HSTRING::from(publisher.as_ref()),
    )?;

    Ok(pkg.into_iter().collect::<Vec<_>>())
  }
}
