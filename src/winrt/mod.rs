use windows::{
  core::{Ref, Result, HSTRING}, Foundation::Uri, Management::Deployment::{DeploymentOptions, DeploymentProgress, DeploymentResult, PackageManager}
};
use windows_collections::IIterable;
use windows_future::{AsyncOperationProgressHandler, IAsyncOperationWithProgress};

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

    prog.SetProgress(&AsyncOperationProgressHandler::new(|a: Ref<IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>, b: Ref<DeploymentProgress>| {
      Ok(())
    }))?;

    Ok(())
  }
}