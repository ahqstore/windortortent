use windows::{
  core::{Error, Result, HSTRING}, ApplicationModel::Package, Foundation::Uri, Management::Deployment::{DeploymentOptions, PackageManager}
};
use windows_collections::IIterable;

pub struct UWPPackageManager(PackageManager);

impl UWPPackageManager {
  pub fn new() -> Result<Self> {
    Ok(Self(PackageManager::new()?))
  }

  pub async fn install(&self, path: &str) -> Result<()> {
    let path = HSTRING::from(path);

    let uri = Uri::CreateUri(&path)?;

    let prog = self.0.AddPackageAsync(
      &uri,
      &IIterable::<Uri>::from(vec![]), 
      DeploymentOptions::InstallAllResources
    )?;

    let result = prog.await?;

    result.ExtendedErrorCode()?.ok()
  }


  pub async fn remove(&self, path: &str) -> Result<()> {
    let uri = Uri::CreateUri(&HSTRING::from(path))?;
    
    let result = self.0.RemovePackageByUriAsync(
      &uri,
      None
    )?.await?;

    result.ExtendedErrorCode()?.ok()
  }

  pub fn get_intalled_info_sync(&self, app_name: &str, publisher: &str) -> Result<Vec<Package>> {
    let pkg = self.0.FindPackagesByNamePublisher(
      &HSTRING::from(app_name),
      &HSTRING::from(publisher)
    )?;

    Ok(pkg.into_iter()
      .collect::<Vec<_>>())
  }
}